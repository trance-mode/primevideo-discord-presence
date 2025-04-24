# windows.ps1 - PrimeVideo Discord Presence インストーラー（管理者チェック・昇格なし）

# 🚨 管理者権限チェック（昇格せず、案内のみ表示）
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
    [Security.Principal.WindowsBuiltinRole]::Administrator)) {
    Write-Host ""
    Write-Host "🔐 このスクリプトは管理者として実行する必要があります。" -ForegroundColor Red
    Write-Host "💡 PowerShell を「管理者として実行」してください（右クリック→管理者として実行）" -ForegroundColor Yellow
    Write-Host ""
    exit 1
}

Write-Host "📦 Installing PrimeVideo Discord Presence..." -ForegroundColor Cyan

# GitHub Actions CI により自動更新されるバージョン
$version = "1.4.0"
$repoRoot = "C:\Program Files\PrimeVideo Discord Presence"
$zipUrl   = "https://github.com/trance-mode/primevideo-discord-presence/archive/refs/heads/main.zip"
$zipPath  = "$env:TEMP\pvdp.zip"

# 🔽 再インストール対策：古いディレクトリを削除
if (Test-Path $repoRoot) {
    Remove-Item -Path $repoRoot -Recurse -Force
    Write-Host "🧹 Removed old install directory." -ForegroundColor DarkGray
}

# 🔽 ダウンロードと展開
Invoke-WebRequest -Uri $zipUrl -OutFile $zipPath
Expand-Archive -Path $zipPath -DestinationPath $env:TEMP -Force
Move-Item "$env:TEMP\primevideo-discord-presence-main" $repoRoot -Force

# 🔨 Rust バイナリビルド
Push-Location "$repoRoot\native"
Write-Host "🔧 Building Rust binary..." -ForegroundColor Yellow
cargo build --release
Pop-Location

# 🧩 Native Messaging マニフェスト登録
Write-Host "🧩 Registering Native Messaging host..." -ForegroundColor Yellow
$hostManifestDir = "$env:LOCALAPPDATA\Google\Chrome\User Data\NativeMessagingHosts"
New-Item -ItemType Directory -Path $hostManifestDir -Force | Out-Null

$template = Get-Content "$repoRoot\installer\com.pvdp.discord.presence.json" -Raw
$exePath = "$repoRoot\native\target\release\pvdp.exe"
$json = $template -replace "PRIME_BINARY_PATH", $exePath
$json | Set-Content "$hostManifestDir\com.pvdp.discord.presence.json"

Write-Host "✅ Native host installed!" -ForegroundColor Green

# 🧩 Chrome 拡張機能の自動追加（レジストリ）
Write-Host "🧩 Registering Chrome extension..." -ForegroundColor Yellow
$extensionPath = "$repoRoot\extension"
$extensionKey  = "HKCU:\Software\Google\Chrome\Extensions\pvdp-extension"

New-Item -Path $extensionKey -Force | Out-Null
Set-ItemProperty -Path $extensionKey -Name "path"     -Value $extensionPath
Set-ItemProperty -Path $extensionKey -Name "version"  -Value $version
Set-ItemProperty -Path $extensionKey -Name "manifest" -Value "$extensionPath\manifest.json"

Write-Host ""
Write-Host "🎉 Installation complete!" -ForegroundColor Green
Write-Host "🔄 Please restart Chrome to activate the extension." -ForegroundColor Cyan
