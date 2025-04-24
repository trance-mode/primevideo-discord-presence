use std::fs;
use std::path::PathBuf;
use eframe::egui;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use serde_json::Value;
use winreg::enums::*;
use winreg::RegKey;
use webbrowser;

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
                ui.label("✅ Installation completed successfully.");
                ui.label("To enable the extension, please open Chrome's extension page.");

                if ui.button("Open Chrome Extensions Page").clicked() {
                    let _ = webbrowser::open("chrome://extensions");
                }
            }

            if self.failed {
                ui.colored_label(egui::Color32::RED, "❌ Installation failed.");
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

        // === 1. Read version from manifest.json ===
        self.log("Reading version...");
        let manifest_text = fs::read_to_string(&manifest_path)?;
        let manifest_json: Value = serde_json::from_str(&manifest_text)?;
        let version = manifest_json["version"].as_str().unwrap_or("0.0.0");

        // === 2. Copy files ===
        self.log("Removing previous installation...");
        if install_dir.exists() {
            fs::remove_dir_all(&install_dir)?;
        }

        self.log("Copying files...");
        let mut opts = CopyOptions::new();
        opts.overwrite = true;
        opts.copy_inside = true;
        fs::create_dir_all(&install_dir)?;
        copy_dir(&extension_dir, &install_dir, &opts)?;
        fs::copy(exe_dir.join("pvdp.exe"), install_dir.join("pvdp.exe"))?;

        // === 3. Create NativeMessaging manifest dynamically ===
        let nmh_json = format!(r#"{{
    "name": "com.pvdp.discord.presence",
    "description": "PVDP Native Messaging Host",
    "path": "{}\\pvdp.exe",
    "type": "stdio",
    "allowed_origins": ["chrome-extension://com.pvdp.discord.presence/"]
}}"#, install_dir.display());

        fs::write(install_dir.join("com.pvdp.discord.presence.json"), nmh_json)?;

        // === 4. Register NativeMessagingHost ===
        self.log("Registering NativeMessaging host...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (nmh_key, _) = hkcu.create_subkey(
            r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence"
        )?;
        nmh_key.set_value("", &format!(r"{}\com.pvdp.discord.presence.json", install_dir.display()))?;

        // === 5. Register Chrome Extension ===
        self.log("Registering Chrome extension...");
        let (ext_key, _) = hkcu.create_subkey(
            r"Software\Google\Chrome\Extensions\com.pvdp.discord.presence"
        )?;
        ext_key.set_value("path", &format!(r"{}\extension", install_dir.display()))?;
        ext_key.set_value("version", &version)?;
        ext_key.set_value("manifest", &format!(r"{}\extension\manifest.json", install_dir.display()))?;

        self.log("All tasks completed.");
        Ok(())
    }
}
