use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use serde_json::Value;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> Result<()> {
    let exe_dir = env::current_exe()?.parent().unwrap().to_path_buf();
    let install_dir = PathBuf::from(r"C:\Program Files\primevideo-discord-presence");
    let extension_dir = exe_dir.join("extension");
    let manifest_path = extension_dir.join("manifest.json");
    let native_manifest = exe_dir.join("com.pvdp.discord.presence.json");

    // === 1. version 読み取り ===
    let manifest_text = fs::read_to_string(&manifest_path)
        .context("failed to read manifest.json")?;
    let manifest_json: Value = serde_json::from_str(&manifest_text)?;
    let version = manifest_json["version"].as_str().unwrap_or("0.0.0");
    println!("📦 Extension version = {}", version);

    // === 2. ファイルをコピー ===
    if install_dir.exists() {
        println!("🧹 Removing existing install dir...");
        fs::remove_dir_all(&install_dir)?;
    }
    println!("📂 Copying to {}", install_dir.display());
    let mut opts = CopyOptions::new();
    opts.overwrite = true;
    opts.copy_inside = true;
    fs::create_dir_all(&install_dir)?;
    copy_dir(&extension_dir, &install_dir, &opts)?;
    fs::copy(exe_dir.join("pvdp.exe"), install_dir.join("pvdp.exe"))?;
    fs::copy(&native_manifest, install_dir.join("com.pvdp.discord.presence.json"))?;

    // === 3. NativeMessagingHosts レジストリ登録 ===
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let nmh_key = hkcu.create_subkey(r"Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence")?;
    nmh_key.set_value("", &format!(r"{}\com.pvdp.discord.presence.json", install_dir.display()))?;
    println!("🔌 NativeMessagingHost registered");

    // === 4. 拡張機能のレジストリ登録（DADP方式） ===
    let ext_key = hkcu.create_subkey(r"Software\Google\Chrome\Extensions\com.pvdp.discord.presence")?;
    ext_key.set_value("path", &format!(r"{}\extension", install_dir.display()))?;
    ext_key.set_value("version", &version)?;
    ext_key.set_value("manifest", &format!(r"{}\extension\manifest.json", install_dir.display()))?;
    println!("🧩 Extension registered");

    println!("\n✅ Installation completed successfully!");
    Ok(())
}
