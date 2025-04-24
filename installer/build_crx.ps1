# build_crx.ps1 - CRX file build script

$crxName = "primevideo-discord-presence.crx"
$manifestPath = "extension/manifest.json"
$keyPath = "extension/key.pem"

# CRXツールインストール
npm install -g crx3

# CRX秘密鍵をGitHub Secretsから復号して保存
Write-Host "Decoding CRX private key..." -ForegroundColor Yellow
echo "${{ secrets.CRX_PRIVATE_KEY }}" | base64 -d > $keyPath

# CRXファイル作成
Write-Host "Creating .crx file..." -ForegroundColor Yellow
crx3 --pack --key=$keyPath --crx="../$crxName" --manifest=$manifestPath

Write-Host "CRX file created: $crxName" -ForegroundColor Green
