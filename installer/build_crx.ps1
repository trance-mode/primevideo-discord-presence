# build_crx.ps1 - CRXファイルを生成するスクリプト

$ErrorActionPreference = "Stop"

$extensionDir = "extension"
$keyPath      = "key.pem"
$outputCrx    = "pvdp.crx"

if (-Not (Test-Path $keyPath)) {
    throw "❌ key.pem が見つかりません。"
}
if (-Not (Test-Path $extensionDir)) {
    throw "❌ extension ディレクトリが見つかりません。"
}

# Chrome 拡張のパッケージ化には crxmake を使用（Node.js / crx3-tools でも可）
# 今回は PowerShell スクリプト側で node-crx パッケージ等の代替は使っていません

# [ChromeのCRX仕様に沿って圧縮・署名する処理がここに必要（GitHub Actions上でバイナリまたは外部ツール使用）]
Write-Host "⚠️ このスクリプトは GitHub Actions でのみ動作を想定（署名処理はCI上）"
