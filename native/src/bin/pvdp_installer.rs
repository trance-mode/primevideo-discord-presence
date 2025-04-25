use std::fs;
use std::path::PathBuf;
use std::process::Command;
use eframe::{egui, NativeOptions};
use egui::{FontData, FontDefinitions, FontFamily, ViewportBuilder};

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([480.0, 460.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "PVDP Installer",
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
    error_message: Option<String>,
}

impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🐿️ PVDP インストーラー");
            ui.separator();

            if self.logs.is_empty() && !self.finished && !self.failed {
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

            if self.finished {
                ui.colored_label(egui::Color32::GREEN, "✅ インストール完了！");
                if ui.button("🌐 Chrome の拡張ページを開く").clicked() {
                    let _ = Command::new("cmd")
                        .args(["/C", "start chrome chrome://extensions"])
                        .spawn();
                }
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "❌ インストールに失敗しました！");
                if let Some(err) = &self.error_message {
                    ui.label(err);
                }
            }

            if self.finished || self.failed {
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

    fn run_install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let install_dir = PathBuf::from(r"C:\Program Files\primevideo-discord-presence");

        self.log("🧹 旧インストール内容の削除を試みます...");
        if install_dir.exists() {
            match fs::remove_dir_all(&install_dir) {
                Ok(_) => self.log("✔️ 旧フォルダ削除成功"),
                Err(e) => self.log(&format!("⚠️ 削除失敗: {}", e)),
            }
        }

        fs::create_dir_all(&install_dir)?;
        self.log("📁 インストールディレクトリを作成");

        let extension_id = "hjngoljbakohoejlcikpfgfmcdjhgppe";

        self.log("📝 NativeMessaging マニフェストを作成中...");
        let manifest = serde_json::json!({
            "name": "com.pvdp.discord.presence",
            "description": "PVDP Native Host",
            "path": install_dir.join("pvdp.exe"),
            "type": "stdio",
            "allowed_origins": [format!("chrome-extension://{}/", extension_id)]
        });
        let manifest_path = install_dir.join("com.pvdp.discord.presence.json");

        match fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?) {
            Ok(_) => self.log("✅ NativeMessaging マニフェスト生成完了"),
            Err(e) => return Err(Box::new(e)),
        }

        self.log("🪟 レジストリへ登録中...");
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
            self.log("✅ レジストリ登録完了");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("レジストリ登録失敗: {}", stderr).into());
        }

        Ok(())
    }
}
