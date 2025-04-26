// === src/bin/pvdp_uninstaller.rs ===

use std::fs;
use std::fs::OpenOptions;
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
        viewport: ViewportBuilder::default().with_inner_size([460.0, 440.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "🗑️ PVDP アンインストーラー",
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
            fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "jp".to_string());
            fonts.families.entry(FontFamily::Monospace).or_default().push("jp".to_string());
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
                ui.colored_label(egui::Color32::GREEN, "✅ アンインストール完了！");
                ui.separator();
                ui.label("⚠️ 拡張機能リストに残っている場合、Chromeの拡張機能ページでゴミ箱を押して削除してください！");
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
        let pvdp_exe = install_dir.join("pvdp.exe");
        let extension_id = "hjngoljbakohoejlcikpfgfmcdjhgppe";

        self.log("🔎 pvdp.exe の実行状態を確認中...");
        if self.is_pvdp_running()? {
            self.log("⚠️ pvdp.exe が起動中です。終了処理を試みます...");
            let kill = Command::new("cmd")
                .args(["/C", "taskkill /F /IM pvdp.exe"])
                .output()?;

            if kill.status.success() {
                self.log("🛑 pvdp.exe を正常に終了しました");
            } else {
                let stderr = String::from_utf8_lossy(&kill.stderr);
                return Err(format!(
                    "❌ pvdp.exe の終了に失敗しました。\n{}\n手動で終了してから再実行してください。",
                    stderr
                ).into());
            }
        } else {
            self.log("✅ pvdp.exe は起動していません。");
        }

        self.log("🧹 インストールフォルダを削除中...");
        if install_dir.exists() {
            // 🛑 ファイルロックも確認してから削除
            if self.is_file_locked(&pvdp_exe)? {
                return Err("❌ pvdp.exeがまだ使用中です。手動で終了してから再実行してください。".into());
            }

            match fs::remove_dir_all(&install_dir) {
                Ok(_) => {
                    self.log("✔️ フォルダ削除成功");
                }
                Err(e) => {
                    return Err(format!("❌ フォルダ削除失敗: {}", e).into());
                }
            }
        } else {
            self.log("ℹ️ インストール済みフォルダが見つかりません");
        }

        self.log("🪟 レジストリキーを削除中...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let _ = hkcu.delete_subkey(r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence");
        let _ = hkcu.delete_subkey(&format!("Software\\Google\\Chrome\\Extensions\\{}", extension_id));
        self.log("✔️ レジストリ削除完了");

        self.log("🎉 アンインストール完了！");
        Ok(())
    }

    fn is_pvdp_running(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let output = Command::new("cmd")
            .args(["/C", "tasklist /FI \"IMAGENAME eq pvdp.exe\""])
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str
            .lines()
            .any(|line| line.to_lowercase().contains("pvdp.exe")))
    }

    fn is_file_locked(&self, path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Ok(false);
        }
        match OpenOptions::new().write(true).open(path) {
            Ok(_) => Ok(false),
            Err(_) => Ok(true),
        }
    }
}
