# windows.ps1 - PrimeVideo Discord Presence インストーラー（.crx 自動登録対応）

Write-Host "📦 Installing PrimeVideo Discord Presence (.crx)" -ForegroundColor Cyan

# ✅ 管理者権限チェック（昇格なし）
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
  ).IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator)) {
    Write-Host "🔒 管理者として実行してください。" -ForegroundColor Red
    Write-Host "👉 PowerShell を右クリック →『管理者として実行』してください。" -ForegroundColor Yellow
    exit 1
}

# 📁 インストール先（Program Files）
$installPath = "C:\Program Files\PrimeVideo Discord Presence"
New-Item -ItemType Directory -Force -Path $installPath | Out-Null

# 📦 GitHub Release から最新バージョンの .crx をダウンロード
$version = "1.4.0" # 本来は CI で埋め込み
$repo    = "trance-mode/primevideo-discord-presence"
$crxUrl  = "https://github.com/$repo/releases/download/v$version/primevideo-discord-presence.crx"
$crxPath = "$installPath\primevideo-discord-presence.crx"

Write-Host "🌐 Downloading .crx from GitHub..." -ForegroundColor Yellow
Invoke-WebRequest -Uri $crxUrl -OutFile $crxPath

# ✅ Chrome 拡張のレジストリ登録（.crx）
Write-Host "🧩 Registering extension (.crx)..." -ForegroundColor Yellow
$extensionId = "pvdp-extension"
$updateUrl   = "https://clients2.google.com/service/update2/crx"
$extKey      = "HKCU:\Software\Google\Chrome\Extensions\$extensionId"

New-Item -Path $extKey -Force | Out-Null
Set-ItemProperty -Path $extKey -Name "update_url" -Value $updateUrl

Write-Host ""
Write-Host "🎉 Installation complete!" -ForegroundColor Green
Write-Host "🔄 Chromeを再起動すると拡張が自動的に有効化されます。" -ForegroundColor Cyan
