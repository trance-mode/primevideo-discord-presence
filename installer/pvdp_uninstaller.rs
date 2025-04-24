use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("🧹 Uninstalling PrimeVideo Discord Presence...");

    // 拡張機能のレジストリ削除
    let reg_key = r"HKCU\Software\Google\Chrome\Extensions\pvdp-extension";
    let reg_del_cmd = format!(
        "if (Test-Path '{}') {{ Remove-Item -Path '{}' -Recurse -Force; Write-Host '🗑️ Removed Chrome extension registry entry.' }} else {{ Write-Host 'ℹ️ No extension registry entry found.' }}",
        reg_key, reg_key
    );
    let _ = Command::new("powershell")
        .args(["-Command", &reg_del_cmd])
        .status();

    // Native Messaging マニフェスト削除
    let manifest_path = PathBuf::from(env::var("LOCALAPPDATA").unwrap())
        .join("Google/Chrome/User Data/NativeMessagingHosts/com.pvdp.discord.presence.json");
    if manifest_path.exists() {
        let _ = fs::remove_file(&manifest_path);
        println!("🗑️ Removed native messaging manifest.");
    } else {
        println!("ℹ️ No native messaging manifest found.");
    }

    // TEMPフォルダ内の作業ディレクトリ削除
    let repo_root = env::temp_dir().join("primevideo-discord-presence");
    if repo_root.exists() {
        let _ = fs::remove_dir_all(&repo_root);
        println!("🧹 Removed working directory: {}", repo_root.display());
    } else {
        println!("ℹ️ No temporary repo directory found.");
    }

    println!("\n✅ Uninstallation complete.");
}
