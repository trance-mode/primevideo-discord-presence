use std::fs;
use std::path::Path;
use eframe::egui;
use winreg::{enums::*, RegKey};

#[link(name = "shell32")]
extern "system" {
    fn IsUserAnAdmin() -> i32;
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "PVDP アンインストーラー",
        options,
        Box::new(|cc| {
            // 日本語フォントの設定
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

            Box::new(UninstallerApp::default())
        }),
    );
}

#[derive(Default)]
struct UninstallerApp {
    logs: Vec<String>,
    finished: bool,
    failed: bool,
    error_message: Option<String>,
    checked_admin: bool,
}

impl eframe::App for UninstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🗑️ PVDP アンインストーラー");
            ui.separator();

            if !self.checked_admin {
                unsafe {
                    if IsUserAnAdmin() == 0 {
                        self.fail("⚠️ 管理者として実行してください。");
                    }
                }
                self.checked_admin = true;
            }

            if self.logs.is_empty() && !self.finished && !self.failed {
                if let Err(e) = self.run_uninstall() {
                    self.fail(&format!("❌ エラー: {}", e));
                } else {
                    self.finished = true;
                }
                ctx.request_repaint();
            }

            for log in &self.logs {
                ui.label(log);
            }

            if self.finished {
                ui.colored_label(egui::Color32::GREEN, "✅ アンインストールが完了しました。");
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "❌ アンインストールに失敗しました。");
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

impl UninstallerApp {
    fn log(&mut self, msg: &str) {
        self.logs.push(format!("🔹 {}", msg));
    }

    fn fail(&mut self, msg: &str) {
        self.failed = true;
        self.error_message = Some(msg.to_string());
        self.log(msg);
    }

    fn run_uninstall(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let install_dir = Path::new(r"C:\Program Files\primevideo-discord-presence");

        self.log("📁 インストールディレクトリの削除中...");
        if install_dir.exists() {
            fs::remove_dir_all(install_dir)?;
            self.log("✅ ディレクトリを削除しました。");
        } else {
            self.log("⚠️ インストールディレクトリが見つかりませんでした。");
        }

        self.log("🧹 レジストリの削除中...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let _ = hkcu.delete_subkey_all(r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence");
        let _ = hkcu.delete_subkey_all(r"Software\Google\Chrome\Extensions\com.pvdp.discord.presence");

        self.log("🎉 アンインストール処理が完了しました。");
        Ok(())
    }
}
