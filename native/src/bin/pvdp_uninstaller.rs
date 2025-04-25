use std::fs;
use std::path::PathBuf;
use std::process::Command;
use eframe::{egui, NativeOptions};
use egui::{FontData, FontDefinitions, FontFamily, ViewportBuilder};
use winreg::enums::*;
use winreg::RegKey;

#[link(name = "shell32")]
extern "system" {
    fn IsUserAnAdmin() -> i32;
}

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([460.0, 420.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "PVDP Uninstaller",
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
                        self.failed = true;
                        self.error_message = Some(concat!(
                            "❌ 管理者として実行してください。\n\n",
                            "▶ 方法：\n",
                            " - アンインストーラーを右クリック →『管理者として実行』\n",
                            " - または、Shift + 右クリック →『管理者として実行』"
                        ).to_string());
                    }
                }
                self.checked_admin = true;
            }

            if self.logs.is_empty() && !self.finished && !self.failed {
                match self.run_uninstall() {
                    Ok(_) => self.finished = true,
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
                ui.colored_label(egui::Color32::GREEN, "✅ アンインストールが完了しました！");
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "❌ アンインストールに失敗しました！");
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
        self.logs.push(format!("🔸 {}", msg));
    }

    fn run_uninstall(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let install_dir = PathBuf::from(r"C:\Program Files\primevideo-discord-presence");

        self.log("🔎 pvdp.exe の実行状態を確認中...");
        let output = Command::new("cmd")
            .args(["/C", "tasklist /FI \"IMAGENAME eq pvdp.exe\""])
            .output()?;
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("pvdp.exe") {
            self.log("⚠️ pvdp.exe が起動中です。終了処理を試みます...");

            let kill = Command::new("cmd")
                .args(["/C", "taskkill /F /IM pvdp.exe"])
                .output()?;

            if kill.status.success() {
                self.log("🛑 pvdp.exe を正常に終了しました。");
            } else {
                self.failed = true;
                self.error_message = Some("❌ pvdp.exe が実行中のためアンインストールできません。\n手動で終了してから再試行してください。".to_string());
                return Ok(()); // 警告表示だけして中断
            }
        } else {
            self.log("✅ pvdp.exe は起動していません。");
        }

        self.log("🧹 インストールフォルダを削除中...");
        if install_dir.exists() {
            fs::remove_dir_all(&install_dir)?;
            self.log("✔️ フォルダ削除成功");
        } else {
            self.log("ℹ️ インストール済みフォルダが見つかりません");
        }

        self.log("🪟 レジストリキーを削除中...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let _ = hkcu.delete_subkey(r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence");
        self.log("✔️ NativeMessagingHost レジストリ削除");

        let _ = hkcu.delete_subkey(r"Software\Google\Chrome\Extensions\com.pvdp.discord.presence");
        self.log("✔️ Extension レジストリ削除");

        self.log("🎉 アンインストール完了！");
        Ok(())
    }
}
