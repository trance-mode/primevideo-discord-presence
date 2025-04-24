use std::fs;
use std::path::PathBuf;
use eframe::egui;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use serde_json::Value;
use winreg::enums::*;
use winreg::RegKey;

#[link(name = "shell32")]
extern "system" {
    fn IsUserAnAdmin() -> i32;
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "PVDP Installer",
        options,
        Box::new(|_cc| Box::new(InstallerApp::default())),
    );
}

#[derive(Default)]
struct InstallerApp {
    logs: Vec<String>,
    finished: bool,
    failed: bool,
    error_message: Option<String>,
    checked_admin: bool,
}

impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PVDP Installer");
            ui.separator();

            if !self.checked_admin {
                unsafe {
                    if IsUserAnAdmin() == 0 {
                        self.failed = true;
                        self.error_message = Some("Please run this installer as Administrator.".to_string());
                    }
                }
                self.checked_admin = true;
            }

            if self.logs.is_empty() && !self.finished && !self.failed {
                match self.run_install() {
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
                ui.label("Installation completed successfully.");
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "Installation failed.");
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

impl InstallerApp {
    fn log(&mut self, message: &str) {
        self.logs.push(message.to_string());
    }

    fn run_install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let exe_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();
        let install_dir = PathBuf::from(r"C:\Program Files\primevideo-discord-presence");
        let extension_dir = exe_dir.join("extension");
        let manifest_path = extension_dir.join("manifest.json");
        let native_manifest = exe_dir.join("com.pvdp.discord.presence.json");

        self.log("Reading version information...");
        let manifest_text = fs::read_to_string(&manifest_path)?;
        let manifest_json: Value = serde_json::from_str(&manifest_text)?;
        let version = manifest_json["version"].as_str().unwrap_or("0.0.0");

        self.log("Removing previous installation...");
        if install_dir.exists() {
            match fs::remove_dir_all(&install_dir) {
                Ok(_) => self.log("Old install directory removed."),
                Err(e) => {
                    self.log(&format!("Failed to remove old install directory: {}", e));
                    return Err(Box::new(e));
                }
            }
        } else {
            self.log("Install directory does not exist.");
        }

        self.log("Copying new files...");
        let mut opts = CopyOptions::new();
        opts.overwrite = true;
        opts.copy_inside = true;
        fs::create_dir_all(&install_dir)?;
        copy_dir(&extension_dir, &install_dir, &opts)?;
        fs::copy(exe_dir.join("pvdp.exe"), install_dir.join("pvdp.exe"))?;
        fs::copy(&native_manifest, install_dir.join("com.pvdp.discord.presence.json"))?;

        self.log("Registering NativeMessaging host...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (nmh_key, _) = hkcu.create_subkey(
            r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence"
        )?;
        nmh_key.set_value(
            "",
            &format!(r"{}\com.pvdp.discord.presence.json", install_dir.display()),
        )?;

        self.log("Registering Chrome extension...");
        let (ext_key, _) = hkcu.create_subkey(
            r"Software\Google\Chrome\Extensions\com.pvdp.discord.presence"
        )?;
        ext_key.set_value("path", &format!(r"{}\extension", install_dir.display()))?;
        ext_key.set_value("version", &version)?;
        ext_key.set_value("manifest", &format!(r"{}\extension\manifest.json", install_dir.display()))?;

        self.log("All tasks completed successfully.");
        Ok(())
    }
}
