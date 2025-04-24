# windows_uninstall.ps1 - PrimeVideo Discord Presence アンインストーラー（Program Files 対応）

# ✅ 管理者権限チェック（昇格）
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
  ).IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator)) {
    Write-Host "🔒 管理者権限で再実行します..." -ForegroundColor Yellow
    Start-Process powershell "-ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

Write-Host "🧹 Uninstalling PrimeVideo Discord Presence..." -ForegroundColor Cyan

# 📁 インストールディレクトリ（Program Files）
$repoRoot = "C:\Program Files\PrimeVideo Discord Presence"

# 🔧 拡張機能のレジストリキーを削除
$extKey = "HKCU:\Software\Google\Chrome\Extensions\pvdp-extension"
if (Test-Path $extKey) {
    Remove-Item -Path $extKey -Recurse -Force
    Write-Host "🗑️ Removed Chrome extension registry entry." -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ No extension registry entry found." -ForegroundColor DarkGray
}

# 🔧 NativeMessaging マニフェスト削除
$manifestPath = "$env:LOCALAPPDATA\Google\Chrome\User Data\NativeMessagingHosts\com.pvdp.discord.presence.json"
if (Test-Path $manifestPath) {
    Remove-Item -Path $manifestPath -Force
    Write-Host "🗑️ Removed native messaging manifest." -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ No native messaging manifest found." -ForegroundColor DarkGray
}

# 🔧 アプリケーション本体削除
if (Test-Path $repoRoot) {
    Remove-Item -Path $repoRoot -Recurse -Force
    Write-Host "🧹 Removed application directory: $repoRoot" -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ Application directory not found." -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "✅ Uninstallation complete." -ForegroundColor Green
