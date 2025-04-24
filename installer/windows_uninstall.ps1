# windows_uninstall.ps1 - PrimeVideo Discord Presence アンインストーラー (.crx方式)

# ✅ 管理者権限チェック（昇格はしない、代わりに案内）
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
  ).IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator)) {
    Write-Host "🔒 管理者として PowerShell を実行してください。" -ForegroundColor Yellow
    Write-Host "右クリック →『管理者として実行』でこのスクリプトを再実行できます。" -ForegroundColor Cyan
    exit 1
}

Write-Host "🧹 Uninstalling PrimeVideo Discord Presence (.crx)..." -ForegroundColor Cyan

# 📁 インストールディレクトリ
$installPath = "C:\Program Files\PrimeVideo Discord Presence"

# 🔧 拡張機能レジストリ削除
$extRegPath = "HKCU:\Software\Google\Chrome\Extensions\pvdp-extension"
if (Test-Path $extRegPath) {
    Remove-Item -Path $extRegPath -Recurse -Force
    Write-Host "🗑️ Removed Chrome extension registry entry." -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ No extension registry found." -ForegroundColor DarkGray
}

# 📁 ディレクトリ削除
if (Test-Path $installPath) {
    Remove-Item -Path $installPath -Recurse -Force
    Write-Host "🧹 Removed install directory: $installPath" -ForegroundColor Yellow
} else {
    Write-Host "ℹ️ Install directory not found." -ForegroundColor DarkGray
}

Write-Host "`n✅ Uninstallation complete." -ForegroundColor Green
