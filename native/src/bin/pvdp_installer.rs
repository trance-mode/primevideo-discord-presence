use std::path::PathBuf;
use eframe::egui;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use include_dir::{include_dir, Dir};
use serde_json::Value;
use winreg::{enums::*, RegKey};

#[link(name = "shell32")]
extern "system" {
    fn IsUserAnAdmin() -> i32;
}

// ← bin から見て ../../extension が正しい
static EXT_DIR: Dir = include_dir!("../../extension");

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "PVDP Installer",
        options,
        Box::new(|cc| {
            // 日本語フォントを設定
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "jp".to_string(),
                egui::FontData::from_static(include_bytes!("../../fonts/NotoSansJP-Regular.ttf")),
            );
            fonts.families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "jp".to_string());
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
    checked_admin: bool,
    show_chrome_button: bool,
}

impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 PVDP インストーラー");
            ui.separator();

            if !self.checked_admin {
                unsafe {
                    if IsUserAnAdmin() == 0 {
                        self.failed = true;
                        self.error_message = Some("❌ 管理者として実行してください。".to_string());
                    }
                }
                self.checked_admin = true;
            }

            if self.logs.is_empty() && !self.finished && !self.failed {
                match self.run_install() {
                    Ok(_) => {
                        self.finished = true;
                        self.show_chrome_button = true;
                    }
                    Err(e) => {
                        self.failed = true;
                        self.error_message = Some(format!("⚠️ {}", e));
                    }
                }
            }

            for log in &self.logs {
                ui.label(log);
            }

            if self.finished {
                ui.colored_label(egui::Color32::GREEN, "✅ インストール完了");
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "❌ インストール失敗");
                if let Some(err) = &self.error_message {
                    ui.label(err);
                }
            }

            if self.show_chrome_button && ui.button("🌐 chrome://extensions を開く").clicked() {
                let _ = std::process::Command::new("cmd")
                    .args(["/C", "start", "chrome", "chrome://extensions"])
                    .spawn();
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
    fn log(&mut self, message: &str) {
        self.logs.push(format!("🔹 {}", message));
    }

    fn run_install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let exe_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();
        let install_dir = PathBuf::from(r"C:\Program Files\primevideo-discord-presence");
        let native_manifest_path = install_dir.join("com.pvdp.discord.presence.json");

        self.log("📖 バージョン情報読み込み中...");
        let manifest_file = EXT_DIR.get_file("manifest.json").ok_or("manifest.json が見つかりません")?;
        let manifest_json: Value = serde_json::from_slice(manifest_file.contents())?;
        let version = manifest_json["version"].as_str().unwrap_or("0.0.0");

        self.log("🧹 旧バージョンを削除中...");
        if install_dir.exists() {
            std::fs::remove_dir_all(&install_dir)?;
        }

        self.log("📂 ディレクトリ作成...");
        std::fs::create_dir_all(&install_dir)?;

        self.log("📦 拡張機能ファイルをコピー中...");
        for file in EXT_DIR.files() {
            let rel_path = file.path();
            let target_path = install_dir.join(rel_path);
            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&target_path, file.contents())?;
            self.log(&format!("✔️ {}", rel_path.display()));
        }

        self.log("📦 pvdp.exe をコピー中...");
        std::fs::copy(exe_dir.join("pvdp.exe"), install_dir.join("pvdp.exe"))?;

        self.log("🧾 NativeMessaging マニフェスト生成...");
        let manifest = format!(
            r#"{{
    "name": "com.pvdp.discord.presence",
    "description": "PVDP native messaging host",
    "path": "{}\\pvdp.exe",
    "type": "stdio",
    "allowed_origins": [
        "chrome-extension://jpnegkohcfkhmnkikhcldjcghjjbnjfc/"
    ]
}}"#,
            install_dir.display()
        );
        std::fs::write(&native_manifest_path, manifest)?;

        self.log("🧠 レジストリに NativeMessagingHost 登録...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (nmh_key, _) = hkcu.create_subkey(
            r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence"
        )?;
        nmh_key.set_value("", &native_manifest_path.display().to_string())?;

        self.log("🧠 Chrome 拡張のレジストリ登録...");
        let (ext_key, _) = hkcu.create_subkey(
            r"Software\Google\Chrome\Extensions\com.pvdp.discord.presence"
        )?;
        ext_key.set_value("path", &format!(r"{}\extension", install_dir.display()))?;
        ext_key.set_value("version", &version)?;
        ext_key.set_value("manifest", &format!(r"{}\extension\manifest.json", install_dir.display()))?;

        self.log("🎉 すべて完了しました！");
        Ok(())
    }
}
