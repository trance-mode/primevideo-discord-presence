
# windows.ps1 - PrimeVideo Discord Presence Installer (.crx + curl.exe)
Write-Host "📦 Installing PrimeVideo Discord Presence (.crx)" -ForegroundColor Cyan

$version = "1.4.0"
$repoRoot = "C:\Program Files\PrimeVideo Discord Presence"
$crxName = "primevideo-discord-presence.crx"
$crxUrl  = "https://github.com/trance-mode/primevideo-discord-presence/releases/download/v$version/$crxName"
$crxPath = "$env:TEMP\$crxName"

# 🚫 管理者権限でなければ中止
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
    [Security.Principal.WindowsBuiltinRole]::Administrator)) {
    Write-Host "🔒 管理者権限で実行してください（右クリック → 管理者として実行）" -ForegroundColor Red
    exit 1
}

# 📂 インストールディレクトリ作成
if (!(Test-Path $repoRoot)) {
    New-Item -ItemType Directory -Path $repoRoot | Out-Null
}

# 🌐 .crx をダウンロード（curl.exe 利用）
Write-Host "🌐 Downloading .crx from GitHub..." -ForegroundColor Yellow
$curlCmd = "curl.exe -L -o `"$crxPath`" `"$crxUrl`""
Invoke-Expression $curlCmd

if (!(Test-Path $crxPath)) {
    Write-Host "❌ Failed to download .crx file" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Download complete: $crxPath" -ForegroundColor Green

# 📁 コピーして保存
Copy-Item -Path $crxPath -Destination "$repoRoot\$crxName" -Force

# 🔧 Chrome 拡張のレジストリ登録（自動追加方式）
Write-Host "🧩 Registering extension (.crx)..." -ForegroundColor Yellow
$regKey = "HKCU:\Software\Google\Chrome\Extensions\pvdp-extension"
New-Item -Path $regKey -Force | Out-Null
Set-ItemProperty -Path $regKey -Name "update_url" -Value "https://clients2.google.com/service/update2/crx"

Write-Host ""
Write-Host "🎉 Installation complete!" -ForegroundColor Green
Write-Host "🔄 Please restart Chrome. Extension will be auto-installed via update_url." -ForegroundColor Cyan
