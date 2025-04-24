# windows_uninstall.ps1 - PrimeVideo Discord Presence アンインストーラー（.crx対応 / 管理者権限前提）

Write-Host "🧹 Uninstalling PrimeVideo Discord Presence (.crx)" -ForegroundColor Cyan

# 📁 インストールディレクトリ
$installDir = "C:\Program Files\PrimeVideo Discord Presence"

# 🔧 Chrome 拡張機能（.crx）レジストリ削除
$extensionKey = "HKCU:\Software\Google\Chrome\Extensions\pvdp-extension"
if (Test-Path $extensionKey) {
    Remove-Item -Path $extensionKey -Recurse -Force
    Write-Host "🗑️ Removed .crx extension registry entry." -ForegroundColor Yellow
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

# 📁 アプリケーション本体ディレクトリの削除
if (Test-Path $installDir) {
    Remove-Item -Path $installDir -Recurse -Force
    Write-Host "🧹 Removed installation directory: $installDir" -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ Installation directory not found." -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "✅ Uninstallation complete." -ForegroundColor Green
