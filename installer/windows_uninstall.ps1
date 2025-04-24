# windows_uninstall.ps1 - PrimeVideo Discord Presence アンインストーラー

Write-Host "🧹 Uninstalling PrimeVideo Discord Presence..." -ForegroundColor Cyan

# 拡張機能のレジストリキーを削除
$extKey = "HKCU:\Software\Google\Chrome\Extensions\pvdp-extension"
if (Test-Path $extKey) {
    Remove-Item -Path $extKey -Recurse -Force
    Write-Host "🗑️ Removed Chrome extension registry entry." -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ No extension registry entry found." -ForegroundColor DarkGray
}

# NativeMessaging マニフェスト削除
$manifestPath = "$env:LOCALAPPDATA\Google\Chrome\User Data\NativeMessagingHosts\com.pvdp.discord.presence.json"
if (Test-Path $manifestPath) {
    Remove-Item -Path $manifestPath -Force
    Write-Host "🗑️ Removed native messaging manifest." -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ No native messaging manifest found." -ForegroundColor DarkGray
}

# 一時ディレクトリ削除（ビルドファイルなど）
$repoRoot = "$env:TEMP\primevideo-discord-presence"
if (Test-Path $repoRoot) {
    Remove-Item -Path $repoRoot -Recurse -Force
    Write-Host "🧹 Removed working directory: $repoRoot" -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ No temporary repo directory found." -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "✅ Uninstallation complete." -ForegroundColor Green
