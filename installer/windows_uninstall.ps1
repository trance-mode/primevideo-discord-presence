# windows_uninstall.ps1 - PrimeVideo Discord Presence アンインストーラー（Program Files 対応）

# ✅ 管理者権限チェック（昇格付き）
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()
    ).IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator)) {
    Write-Host ""
    Write-Host "🔐 このスクリプトは管理者として実行する必要があります。" -ForegroundColor Red
    Write-Host "💡 PowerShell を『管理者として実行』してください（右クリック→管理者として実行）" -ForegroundColor Yellow
    Write-Host ""
    exit 1
}

Write-Host "🧹 Uninstalling PrimeVideo Discord Presence..." -ForegroundColor Cyan

# 📁 アプリ本体のインストール先（Program Files）
$repoRoot = "C:\Program Files\PrimeVideo Discord Presence"

# 🔧 拡張機能のレジストリキーを削除
$extKey = "HKCU:\Software\Google\Chrome\Extensions\pvdp-extension"
if (Test-Path $extKey) {
    Remove-Item -Path $extKey -Recurse -Force
    Write-Host "🗑️ Removed Chrome extension registry entry." -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ No extension registry entry found." -ForegroundColor DarkGray
}

# 🔧 Native Messaging マニフェスト削除
$manifestPath = "$env:LOCALAPPDATA\Google\Chrome\User Data\NativeMessagingHosts\com.pvdp.discord.presence.json"
if (Test-Path $manifestPath) {
    Remove-Item -Path $manifestPath -Force
    Write-Host "🗑️ Removed native messaging manifest." -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ No native messaging manifest found." -ForegroundColor DarkGray
}

# 🔧 アプリ本体を削除
if (Test-Path $repoRoot) {
    Remove-Item -Path $repoRoot -Recurse -Force
    Write-Host "🧹 Removed application directory: $repoRoot" -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ Application directory not found." -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "✅ Uninstallation complete." -ForegroundColor Green
