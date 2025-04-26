// === src/bin/pvdp_installer.rs ===

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use eframe::{egui, NativeOptions};
use egui::{FontData, FontDefinitions, FontFamily, ViewportBuilder};
use include_dir::{include_dir, Dir};

#[link(name = "shell32")]
extern "system" {
    fn IsUserAnAdmin() -> i32;
}

// ✅ Cargo.tomlのversionから自動取得
const PVDP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

// ✅ pvdp.exeバイナリを埋め込み
const PVDP_EXE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/target/release/pvdp.exe"));
// ✅ extension/ディレクトリを埋め込み
const EXTENSION_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/extension");

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([480.0, 600.0]), // ✅ 高さ600pxに拡大
        ..Default::default()
    };

    let _ = eframe::run_native(
        "🐿️ PVDP Installer",
        options,
        Box::new(|cc| {
            let mut fonts = FontDefinitions::default();
            fonts.font_data.insert(
                "jp".to_string(),
                FontData::from_static(include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/fonts/NotoSansJP-Regular.ttf"
                ))),
            );
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "jp".to_string());
            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .push("jp".to_string());
            cc.egui_ctx.set_fonts(fonts);

            Box::new(InstallerApp::default())
        }),
    );
}

#[derive(Default)]
struct InstallerApp {
    logs: Vec<String>,
    finished: bool,
    failed: bool,
    already_installed: bool,
    error_message: Option<String>,
    checked_admin: bool,
}

impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🐿️ PVDP インストーラー");
            ui.label(format!("バージョン {}", PVDP_VERSION));
            ui.separator();

            if !self.checked_admin {
                unsafe {
                    if IsUserAnAdmin() == 0 {
                        self.failed = true;
                        self.error_message = Some(concat!(
                            "❌ 管理者として実行してください。\n\n",
                            "▶ 方法：\n",
                            " - インストーラーを右クリック →『管理者として実行』\n",
                            " - または、Shift + 右クリック →『管理者として実行』"
                        ).to_string());
                    }
                }
                self.checked_admin = true;
            }

            if !self.failed && !self.finished && self.logs.is_empty() && !self.already_installed {
                if Self::install_dir().exists() {
                    self.already_installed = true;
                    self.error_message = Some(
                        "⚠️ すでにインストールされています。アンインストール後に再度お試しください。".to_string()
                    );
                }
            }

            if self.logs.is_empty() && !self.finished && !self.failed && !self.already_installed {
                match self.run_install() {
                    Ok(_) => self.finished = true,
                    Err(e) => {
                        self.failed = true;
                        self.error_message = Some(format!("⚠️ インストール失敗: {}", e));
                    }
                }
            }

            for log in &self.logs {
                ui.label(log);
            }

            if self.already_installed {
                ui.colored_label(egui::Color32::YELLOW, "⚠️ すでにインストールされています");
                if let Some(err) = &self.error_message {
                    ui.label(err);
                }
            }

            if self.finished {
                ui.colored_label(egui::Color32::GREEN, "✅ インストール完了！");
                ui.label("🌟 Chromeが起動しました！");
                ui.label("🌟 Chromeのアドレスバーに「chrome://extensions」と入力して拡張機能ページを開いてください！");
                ui.separator();
                ui.label("✋ 手動追加方法 ✋");
                ui.label("1. 右上の「開発者モード」をオンにします");
                ui.label("2. 「パッケージ化されていない拡張機能を読み込む」を押します");
                ui.label("3. インストーラーが開いた extension フォルダを選択してください！");
                ui.label("追加完了後プライム動画を再生するとプレゼンスが表示されます");

                if ui.button("🌐 Chromeを開く & extensionフォルダを開く").clicked() {
                    InstallerApp::open_chrome_and_extension_folder(self);
                }
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "❌ インストールに失敗しました！");
                if let Some(err) = &self.error_message {
                    ui.label(err);
                }
            }

            if self.finished || self.failed || self.already_installed {
                if ui.button("❎ 閉じる").clicked() {
                    std::process::exit(0);
                }
            }
        });
    }
}

impl InstallerApp {
    fn log(&mut self, msg: &str) {
        self.logs.push(format!("🔸 {}", msg));
    }

    fn install_dir() -> PathBuf {
        PathBuf::from(r"C:\Program Files\primevideo-discord-presence")
    }

    fn open_chrome_and_extension_folder(&mut self) {
        // Chromeを起動
        let _ = Command::new("cmd")
            .args(["/C", "start", "chrome"])
            .spawn();

        // インストールフォルダを開く
        let install_dir = Self::install_dir();
        if install_dir.exists() {
            let _ = Command::new("explorer")
                .arg(install_dir)
                .spawn();
        } else {
            self.log("⚠️ インストールフォルダが見つかりませんでした。");
        }
    }

    fn run_install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let install_dir = Self::install_dir();
        let extension_id = "hjngoljbakohoejlcikpfgfmcdjhgppe";

        self.log("🧹 前のインストールを削除中...");
        if install_dir.exists() {
            match fs::remove_dir_all(&install_dir) {
                Ok(_) => self.log("✔️ 削除成功"),
                Err(e) => self.log(&format!("⚠️ 削除失敗: {}", e)),
            }
        }

        self.log("📂 インストールディレクトリ作成中...");
        fs::create_dir_all(&install_dir)?;

        self.log("📄 pvdp.exe を書き込み中...");
        fs::write(install_dir.join("pvdp.exe"), PVDP_EXE_BYTES)?;

        self.log("📦 拡張機能ファイルを展開中...");
        let ext_dir = install_dir.join("extension");
        fs::create_dir_all(&ext_dir)?;
        EXTENSION_DIR.extract(&ext_dir)
            .map_err(|e| format!("拡張機能展開失敗: {}", e))?;
        self.log(&format!("📁 拡張機能展開先: \"{}\"", ext_dir.display()));
        self.log("✅ 拡張機能ファイル展開完了");

        self.log("📄 NativeMessaging マニフェスト JSON を構築中...");
        let manifest = serde_json::json!({
            "name": "com.pvdp.discord.presence",
            "description": "PVDP Native Host",
            "path": install_dir.join("pvdp.exe"),
            "type": "stdio",
            "allowed_origins": [format!("chrome-extension://{}/", extension_id)],
        });

        let manifest_path = install_dir.join("com.pvdp.discord.presence.json");
        self.log(&format!("📄 マニフェストファイル書き込み中: \"{}\"", manifest_path.display()));
        fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
        self.log("✅ NativeMessaging マニフェスト生成完了");

        self.log("🪟 NativeMessaging レジストリ登録中...");
        let output = Command::new("reg")
            .args([
                "add",
                "HKCU\\Software\\Google\\Chrome\\NativeMessagingHosts\\com.pvdp.discord.presence",
                "/t",
                "REG_SZ",
                "/d",
                &manifest_path.to_string_lossy(),
                "/f",
            ])
            .output()?;

        if output.status.success() {
            self.log("✅ NativeMessaging レジストリ登録完了");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("❌ NativeMessagingレジストリ登録失敗: {}", stderr).into());
        }

        Ok(())
    }
}
