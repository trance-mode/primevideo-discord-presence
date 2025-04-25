use std::path::PathBuf;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use include_dir::{include_dir, Dir};
use serde_json::Value;
use winreg::enums::*;
use winreg::RegKey;
use eframe::egui::ViewportBuilder;

#[link(name = "shell32")]
extern "system" {
    fn IsUserAnAdmin() -> i32;
}

static EXT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/extension");

fn main() {
    // アイコンを設定したい場合は以下を有効化
    // let icon_bytes = include_bytes!("../../assets/pvdp.ico");
    // let icon = Some(eframe::icon_data::from_ico(icon_bytes).expect("ICO読み込み失敗"));

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([480.0, 500.0]),
        // icon_data: icon,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "PVDP Installer",
        options,
        Box::new(|cc| {
            let mut fonts = FontDefinitions::default();
            fonts.font_data.insert(
                "jp".to_string(),
                FontData::from_static(include_bytes!("../../fonts/NotoSansJP-Regular.ttf")),
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
                        self.error_message = Some(concat!(
                            "❌ 管理者として実行してください。\n\n",
                            "▶ 方法：\n",
                            " - インストーラーを右クリック →『管理者として実行』を選択\n",
                            " - または、Shift + 右クリック →『管理者として実行』"
                        ).to_string());
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
                        self.error_message = Some(format!("⚠️ エラー: {}", e));
                    }
                }
            }

            for log in &self.logs {
                ui.label(log);
            }

            if self.finished {
                ui.colored_label(egui::Color32::GREEN, "✅ インストール完了！");
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "❌ インストールに失敗しました！");
                if let Some(err) = &self.error_message {
                    ui.label(err);
                }
            }

            if self.show_chrome_button && ui.button("🧩 拡張機能ページを開く").clicked() {
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
    fn log(&mut self, msg: &str) {
        self.logs.push(format!("🔸 {}", msg));
    }

    fn run_install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let exe_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();
        let install_dir = PathBuf::from(r"C:\Program Files\primevideo-discord-presence");
        let native_manifest_path = install_dir.join("com.pvdp.discord.presence.json");

        self.log("📖 バージョン情報を読み込み中...");
        let manifest_file = EXT_DIR.get_file("manifest.json").ok_or("manifest.json が見つかりません")?;
        let manifest_json: Value = serde_json::from_slice(manifest_file.contents())?;
        let version = manifest_json["version"].as_str().unwrap_or("0.0.0");

        self.log("🧹 前のインストールを削除中...");
        if install_dir.exists() {
            std::fs::remove_dir_all(&install_dir)?;
        }

        self.log("📂 インストールディレクトリ作成中...");
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

        self.log("📄 pvdp.exe をコピー中...");
        std::fs::copy(exe_dir.join("pvdp.exe"), install_dir.join("pvdp.exe"))?;

        self.log("🧾 NativeMessaging マニフェストを生成中...");
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

        self.log("📝 レジストリへ登録中...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (nmh_key, _) = hkcu.create_subkey(
            r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence"
        )?;
        nmh_key.set_value("", &native_manifest_path.display().to_string())?;

        let (ext_key, _) = hkcu.create_subkey(
            r"Software\Google\Chrome\Extensions\com.pvdp.discord.presence"
        )?;
        ext_key.set_value("path", &format!(r"{}\extension", install_dir.display()))?;
        ext_key.set_value("version", &version)?;
        ext_key.set_value("manifest", &format!(r"{}\extension\manifest.json", install_dir.display()))?;

        self.log("🎉 インストール完了！");
        Ok(())
    }
}
