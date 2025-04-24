use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    println!("📦 Installing PrimeVideo Discord Presence(PVDP)...");

    let temp = env::temp_dir();
    let repo_root = temp.join("primevideo-discord-presence");
    let zip_path = temp.join("pvdp.zip");
    let repo_url = "https://github.com/trance-mode/primevideo-discord-presence/archive/refs/heads/main.zip";

    // ダウンロード
    if !Command::new("powershell")
        .args(["-Command", &format!("Invoke-WebRequest '{}' -OutFile '{}'", repo_url, zip_path.display())])
        .status()
        .expect("❌ Failed to download zip")
        .success()
    {
        eprintln!("❌ Download failed");
        return;
    }

    // 展開
    if !Command::new("powershell")
        .args(["-Command", &format!("Expand-Archive -Path '{}' -DestinationPath '{}' -Force", zip_path.display(), temp.display())])
        .status()
        .expect("❌ Failed to extract zip")
        .success()
    {
        eprintln!("❌ Extraction failed");
        return;
    }

    let unpacked = temp.join("primevideo-discord-presence-main");
    if repo_root.exists() {
        let _ = fs::remove_dir_all(&repo_root);
    }
    fs::rename(&unpacked, &repo_root).expect("❌ Failed to move project folder");

    // Rust ビルド
    let native_dir = repo_root.join("native");
    let build = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&native_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("❌ Cargo build failed");

    if !build.success() {
        eprintln!("❌ Build failed");
        return;
    }

    let exe_path = native_dir.join("target/release/prime_video_discord_presence.exe");

    // NativeMessaging マニフェスト登録
    let host_manifest_dir = PathBuf::from(env::var("LOCALAPPDATA").unwrap())
        .join("Google/Chrome/User Data/NativeMessagingHosts");
    fs::create_dir_all(&host_manifest_dir).expect("❌ Failed to create manifest directory");

    let json_template_path = repo_root.join("installer/resources/com.pvdp.discord.presence.json");
    let json_template = fs::read_to_string(&json_template_path).expect("❌ Failed to read manifest template");
    let replaced = json_template.replace("PRIME_BINARY_PATH", &exe_path.display().to_string());
    let json_dest = host_manifest_dir.join("com.pvdp.discord.presence.json");
    fs::write(&json_dest, replaced).expect("❌ Failed to write manifest file");

    // Chrome 拡張のレジストリ登録
    let extension_path = repo_root.join("extension");
    let reg_key = r"HKCU\Software\Google\Chrome\Extensions\pvdp-extension";
    let manifest_path = extension_path.join("manifest.json");
    let version = "1.4.0";

    let reg_cmd = format!(
        "New-Item -Path '{}' -Force | Out-Null; \
         Set-ItemProperty -Path '{}' -Name 'path' -Value '{}'; \
         Set-ItemProperty -Path '{}' -Name 'version' -Value '{}'; \
         Set-ItemProperty -Path '{}' -Name 'manifest' -Value '{}'",
        reg_key,
        reg_key,
        extension_path.display(),
        reg_key,
        version,
        reg_key,
        manifest_path.display()
    );

    if !Command::new("powershell")
        .args(["-Command", &reg_cmd])
        .status()
        .expect("❌ Failed to write registry")
        .success()
    {
        eprintln!("❌ Failed to register Chrome extension");
        return;
    }

    println!("\n✅ Installation complete!");
    println!("🔄 Chromeを再起動すると拡張が自動的に追加されます（必要に応じて手動で有効化）");
}
