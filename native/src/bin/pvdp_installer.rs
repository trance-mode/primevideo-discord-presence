use std::path::PathBuf;
use eframe::egui;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use include_dir::{include_dir, Dir};
use serde_json::Value;
use winreg::enums::*;
use winreg::RegKey;

#[link(name = "shell32")]
extern "system" {
    fn IsUserAnAdmin() -> i32;
}

static EXT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../extension");

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
    show_chrome_button: bool,
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
                    Ok(_) => {
                        self.finished = true;
                        self.show_chrome_button = true;
                    }
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

            if self.show_chrome_button && ui.button("Open chrome://extensions").clicked() {
                let _ = std::process::Command::new("cmd")
                    .args(["/C", "start", "chrome", "chrome://extensions"])
                    .spawn();
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
        let native_manifest_path = install_dir.join("com.pvdp.discord.presence.json");

        self.log("Reading version information...");
        let manifest_file = EXT_DIR.get_file("manifest.json").ok_or("manifest.json not found")?;
        let manifest_json: Value = serde_json::from_slice(manifest_file.contents())?;
        let version = manifest_json["version"].as_str().unwrap_or("0.0.0");

        self.log("Removing previous installation...");
        if install_dir.exists() {
            std::fs::remove_dir_all(&install_dir)?;
        }

        self.log("Copying embedded files...");
        for entry in EXT_DIR.find("**/*").unwrap() {
            if let Some(file) = entry.as_file() {
                let rel_path = file.path();
                let target_path = install_dir.join(rel_path);
                if let Some(parent) = target_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&target_path, file.contents())?;
            }
        }

        std::fs::copy(exe_dir.join("pvdp.exe"), install_dir.join("pvdp.exe"))?;

        self.log("Generating NativeMessaging manifest...");
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

        self.log("Registering NativeMessaging host...");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (nmh_key, _) = hkcu.create_subkey(
            r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence"
        )?;
        nmh_key.set_value("", &native_manifest_path.display().to_string())?;

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
