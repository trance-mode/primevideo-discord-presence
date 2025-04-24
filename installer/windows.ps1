# windows.ps1 - PrimeVideo Discord Presence インストーラー（v1.4.0+ / CI対応）

Write-Host "📦 Installing PrimeVideo Discord Presence..." -ForegroundColor Cyan

# バージョンは GitHub Actions CI により自動反映されます
$version = "1.4.0"

$repoRoot = "$env:TEMP\primevideo-discord-presence"
$zipUrl   = "https://github.com/trance-mode/primevideo-discord-presence/archive/refs/heads/main.zip"
$zipPath  = "$env:TEMP\pvdp.zip"

# 🔽 ダウンロードと展開
Invoke-WebRequest -Uri $zipUrl -OutFile $zipPath
Expand-Archive -Path $zipPath -DestinationPath $env:TEMP -Force
Move-Item "$env:TEMP\primevideo-discord-presence-main" $repoRoot -Force

# 🔨 Rust バイナリビルド
Push-Location "$repoRoot\native"
Write-Host "🔧 Building Rust binary..." -ForegroundColor Yellow
cargo build --release
Pop-Location

# 🧩 Native Messaging マニフェストの登録
Write-Host "🧩 Registering Native Messaging host..." -ForegroundColor Yellow
$hostManifestDir = "$env:LOCALAPPDATA\Google\Chrome\User Data\NativeMessagingHosts"
New-Item -ItemType Directory -Path $hostManifestDir -Force | Out-Null

$template = Get-Content "$repoRoot\installer\resources\com.pvdp.discord.presence.json" -Raw
$exePath = "$repoRoot\native\target\release\pvdp.exe"
$json = $template -replace "PRIME_BINARY_PATH", $exePath
$json | Set-Content "$hostManifestDir\com.pvdp.discord.presence.json"

Write-Host "✅ Native host installed!" -ForegroundColor Green

# 🔧 Chrome 拡張機能の自動追加（レジストリ登録）
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
