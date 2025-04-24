use std::fs;
use std::path::PathBuf;
use eframe::egui;
use winreg::enums::*;
use winreg::RegKey;

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
                ui.label("Uninstallation completed successfully.");
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "Uninstallation failed.");
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
        let install_dir = PathBuf::from(r"C:\Program Files\primevideo-discord-presence");

        self.log("Removing installed files...");
        if install_dir.exists() {
            match fs::remove_dir_all(&install_dir) {
                Ok(_) => self.log("Install directory removed."),
                Err(e) => {
                    self.log(&format!("Failed to remove install directory: {}", e));
                    return Err(Box::new(e));
                }
            }
        } else {
            self.log("Install directory does not exist.");
        }

        self.log("Cleaning registry entries...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        match hkcu.delete_subkey_all(r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence") {
            Ok(_) => self.log("NativeMessaging host key removed."),
            Err(e) => self.log(&format!("Failed to delete NativeMessaging host key: {}", e)),
        }

        match hkcu.delete_subkey_all(r"Software\Google\Chrome\Extensions\com.pvdp.discord.presence") {
            Ok(_) => self.log("Chrome extension key removed."),
            Err(e) => self.log(&format!("Failed to delete Chrome extension key: {}", e)),
        }

        self.log("Uninstall process finished.");
        Ok(())
    }
}