// === Rust main.rs ===（WebSocketログ転送 + トレイ対応 + SinkExt修正済）
use anyhow::Result;
use chrono::{Duration as ChronoDuration, Local};
use discord_sdk as ds;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::sync::broadcast;
use tracing_subscriber::{filter::LevelFilter, prelude::*};
use warp::Filter;
use warp::ws::Message;
use tray_item::{TrayItem, IconSource};

const DISCORD_APP_ID: i64 = 1362378842672730294;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Presence {
    message_type: u8,
    title: String,
    episodes: String,
    total_duration: String,
    current_time: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let (log_tx, _log_rx) = broadcast::channel(500);
    init_logger(log_tx.clone());

    std::thread::spawn(|| {
        if let Err(e) = spawn_tray() {
            eprintln!("トレイ起動失敗: {:?}", e);
        }
    });

    tracing::info!(target: "prime_video_discord_presence", "🟢 Rust アプリ起動");

    let ws_task = tokio::spawn(start_websocket_server(log_tx.clone()));
    let main_task = tokio::spawn(run_once());

    let _ = tokio::join!(ws_task, main_task);
    Ok(())
}

fn spawn_tray() -> Result<()> {
    let mut tray = TrayItem::new("PVDP", IconSource::Resource("IDI_ICON1"))?;
    tray.add_menu_item("終了", || {
        tracing::info!(target: "prime_video_discord_presence", "🛑 トレイから終了");
        std::process::exit(0);
    })?;
    loop {
        std::thread::park();
    }
}

async fn run_once() -> Result<()> {
    let (wheel, handler) = ds::wheel::Wheel::new(Box::new(|e| tracing::error!(?e)));
    let discord = ds::Discord::new(
        ds::DiscordApp::PlainId(DISCORD_APP_ID),
        ds::Subscriptions::ACTIVITY,
        Box::new(handler),
    )?;
    let mut user = wheel.user();
    user.0.changed().await?;
    if let ds::wheel::UserState::Connected(u) = &*user.0.borrow() {
        tracing::info!(target: "prime_video_discord_presence", "✅ Discord 接続: {} ({})", u.username, u.id);
    }
    let client = Arc::new(discord);

    let idle = ds::activity::ActivityBuilder::default()
        .kind(ds::activity::ActivityKind::Watching)
        .details("Prime Video Discord Presence 起動中")
        .state("接続待機中...")
        .assets(ds::activity::Assets::default().large("prime", Some("Prime Video")));
    client.update_activity(idle).await?;

    let mut reader = BufReader::new(tokio::io::stdin());
    loop {
        let mut len_buf = [0u8; 4];
        if reader.read_exact(&mut len_buf).await.is_err() {
            break;
        }
        let len = u32::from_le_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        if reader.read_exact(&mut buf).await.is_err() {
            break;
        }
        let s = String::from_utf8(buf).unwrap_or_default();
        tracing::debug!(target: "prime_video_discord_presence", "📨 raw json: {}", s);

        let v: Value = match serde_json::from_str(&s) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(target: "prime_video_discord_presence", "❌ JSON parse error: {}", e);
                continue;
            }
        };

        let mtype = v.get("message_type").and_then(|v| v.as_u64()).unwrap_or(255) as u8;
        tracing::debug!(target: "prime_video_discord_presence", "📬 message_type = {}", mtype);

        match mtype {
            0 => tracing::debug!(target: "prime_video_discord_presence", "💤 keep-alive ping"),
            3 | 4 | 5 => {
                let msg: Presence = match serde_json::from_value(v) {
                    Ok(m) => m,
                    Err(e) => {
                        tracing::warn!(target: "prime_video_discord_presence", "⚠️ Presence parse error: {}", e);
                        continue;
                    }
                };
                handle(&client, msg).await?;
            }
            _ => tracing::warn!(target: "prime_video_discord_presence", "❓ Unknown message_type: {}", mtype),
        }
    }

    tracing::info!(target: "prime_video_discord_presence", "🛑 アプリ終了");
    Ok(())
}

async fn handle(cli: &ds::Discord, p: Presence) -> Result<()> {
    match p.message_type {
        3 => {
            tracing::debug!(target: "prime_video_discord_presence", "🎮 Presence 更新: {} - {}", p.title, p.episodes);
            let now     = Local::now();
            let current = parse(&p.current_time);
            let total   = parse(&p.total_duration);
            let start   = now - current;
            let end     = start + total;
            let detail = format!("Prime Video: {}", p.title);
            let state  = format!("{} | {}", p.episodes, p.total_duration);
            let act = ds::activity::ActivityBuilder::default()
                .kind(ds::activity::ActivityKind::Playing)
                .details(&detail)
                .state(&state)
                .start_timestamp(start.timestamp())
                .end_timestamp(end.timestamp())
                .assets(ds::activity::Assets::default().large("prime", Some("Prime Video")));
            cli.update_activity(act).await?;
        }
        4 => {
            tracing::debug!(target: "prime_video_discord_presence", "🧹 Clear activity (received from content.js)");
            cli.clear_activity().await?;
        }
        5 => {
            tracing::debug!(target: "prime_video_discord_presence", "🛑 Shutdown received");
            std::process::exit(0);
        }
        _ => tracing::warn!(target: "prime_video_discord_presence", "❓ Unhandled message_type: {}", p.message_type),
    }
    Ok(())
}

fn parse(s: &str) -> ChronoDuration {
    let parts: Vec<&str> = s.split(':').collect();
    let (h, m, sec) = match parts.len() {
        3 => (parts[0], parts[1], parts[2]),
        2 => ("0", parts[0], parts[1]),
        _ => ("0", "0", "0"),
    };
    let h: i64   = h.parse().unwrap_or(0);
    let m: i64   = m.parse().unwrap_or(0);
    let sec: i64 = sec.parse().unwrap_or(0);
    ChronoDuration::seconds(h * 3600 + m * 60 + sec)
}

fn init_logger(log_tx: broadcast::Sender<String>) {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(move || LogSink(log_tx.clone()))
        .with_ansi(false);

    let filter_layer = fmt_layer.with_filter(LevelFilter::DEBUG);

    tracing_subscriber::registry()
        .with(filter_layer)
        .init();
}

struct LogSink(broadcast::Sender<String>);
impl std::io::Write for LogSink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(msg) = std::str::from_utf8(buf) {
            let _ = self.0.send(msg.trim().to_string());
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

async fn start_websocket_server(log_tx: broadcast::Sender<String>) {
    let route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = log_tx.clone();
            ws.on_upgrade(move |socket| async move {
                let (mut tx_ws, _) = socket.split();
                let mut rx = tx.subscribe();
                tracing::info!(target: "prime_video_discord_presence", "🌐 WebSocket client connected");
                while let Ok(msg) = rx.recv().await {
                    if tx_ws.send(Message::text(msg)).await.is_err() {
                        tracing::warn!(target: "prime_video_discord_presence", "❌ WS client disconnected");
                        break;
                    }
                }
            })
        });
    let addr: SocketAddr = ([127, 0, 0, 1], 3012).into();
    warp::serve(route).run(addr).await;
}
