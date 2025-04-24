// Self-contained pvdp_uninstaller.rs (English GUI)
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
        "PVDP Uninstaller",
        options,
        Box::new(|_cc| Box::new(UninstallerApp::default())),
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
            ui.heading("PVDP Uninstaller");
            ui.separator();

            if !self.checked_admin {
                unsafe {
                    if IsUserAnAdmin() == 0 {
                        self.failed = true;
                        self.error_message = Some("Please run this uninstaller as Administrator.".to_string());
                    }
                }
                self.checked_admin = true;
            }

            if self.logs.is_empty() && !self.finished && !self.failed {
                match self.run_uninstall() {
                    Ok(_) => self.finished = true,
                    Err(e) => {
                        self.failed = true;
                        self.error_message = Some(format!("{:?}", e));
                    }
                }
            }

            for log in &self.logs {
                ui.label(log);
            }

            if self.finished {
                ui.label("✅ Uninstallation completed successfully.");
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "❌ Uninstallation failed.");
                if let Some(err) = &self.error_message {
                    ui.label(err);
                }
            }

            if self.finished || self.failed {
                if ui.button("Close").clicked() {
                    std::process::exit(0);
                }
            }
        });
    }
}

impl UninstallerApp {
    fn log(&mut self, message: &str) {
        self.logs.push(message.to_string());
    }

    fn run_uninstall(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let install_dir = Path::new(r"C:\Program Files\primevideo-discord-presence");

        self.log("Removing files...");
        if install_dir.exists() {
            fs::remove_dir_all(install_dir)?;
        } else {
            self.log("Install directory not found.");
        }

        self.log("Removing registry keys...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        let _ = hkcu.delete_subkey_all(r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence");
        let _ = hkcu.delete_subkey_all(r"Software\Google\Chrome\Extensions\com.pvdp.discord.presence");

        self.log("All cleanup tasks completed.");
        Ok(())
    }
}
