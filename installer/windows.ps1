# windows.ps1 - PrimeVideo Discord Presence Installer (.crx + curl.exe + log/version aware)

$ErrorActionPreference = "Stop"

Write-Host "📦 Installing PrimeVideo Discord Presence (.crx)" -ForegroundColor Cyan

# === VERSION AUTO-DETECTION ===
try {
  $version = (Invoke-RestMethod https://api.github.com/repos/trance-mode/primevideo-discord-presence/releases/latest).tag_name
  $version = $version.TrimStart("v")
} catch {
  Write-Host "❌ Failed to fetch latest version info. Falling back to hardcoded version." -ForegroundColor Red
  $version = "1.4.0"
}

Write-Host "📌 Target version: $version" -ForegroundColor DarkCyan

$repoRoot = "C:\Program Files\PrimeVideo Discord Presence"
$crxName = "primevideo-discord-presence.crx"
$crxUrl  = "https://github.com/trance-mode/primevideo-discord-presence/releases/download/v$version/$crxName"
$crxPath = "$env:TEMP\$crxName"

# 🚫 管理者チェック
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
    [Security.Principal.WindowsBuiltinRole]::Administrator)) {
    Write-Host "🔒 管理者権限で実行してください（右クリック → 管理者として実行）" -ForegroundColor Red
    exit 1
}

# 📂 ディレクトリ作成
if (!(Test-Path $repoRoot)) {
    New-Item -ItemType Directory -Path $repoRoot | Out-Null
}

# 🌐 .crx ダウンロード
Write-Host "🌐 Downloading .crx from GitHub..." -ForegroundColor Yellow
$curlCmd = "curl.exe -L -o `"$crxPath`" `"$crxUrl`""
Invoke-Expression $curlCmd

if (!(Test-Path $crxPath)) {
    Write-Host "❌ Failed to download .crx file" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Download complete: $crxPath" -ForegroundColor Green

# 📁 コピー
Copy-Item -Path $crxPath -Destination "$repoRoot\$crxName" -Force

# 🔧 Chrome 拡張登録
Write-Host "🧩 Registering extension (.crx)..." -ForegroundColor Yellow
$regKey = "HKCU:\Software\Google\Chrome\Extensions\pvdp-extension"
New-Item -Path $regKey -Force | Out-Null
Set-ItemProperty -Path $regKey -Name "update_url" -Value "https://clients2.google.com/service/update2/crx"

Write-Host ""
Write-Host "🎉 Installation complete!" -ForegroundColor Green
Write-Host "🔄 Please restart Chrome. Extension will be auto-installed via update_url." -ForegroundColor Cyan
