# Prime Video Discord Presence (PVDP)

🎬 Amazon Prime Video の再生状況を Discord のステータスに表示する、**Chrome 拡張機能 + Rust ネイティブアプリ**です。

*A Chrome Extension + Rust Native Host to display your Amazon Prime Video activity as a Discord Rich Presence status.*

![version](https://img.shields.io/github/v/release/trance-mode/primevideo-discord-presence)
[![Download Installer](https://img.shields.io/badge/Download-pvdp__installer.exe-blue?logo=github)](https://github.com/trance-mode/primevideo-discord-presence/releases/latest/download/pvdp_installer.exe)

---

## 📦 主な機能 / Features

- ✅ Prime Video の再生中／停止中を自動検知
- 🕒 Discord にタイトル・エピソード・残り時間を表示
- 🚀 Rust ネイティブとの高速通信（Chrome Native Messaging）
- 🔌 ログビューア連携（WebSocketでリアルタイムログ表示）
- 🧠 高精度な再生状態判断（UI ボタン検出 + MutationObserver）
- 🎨 ログの色分け / ステータス表示強化（v1.4.0+）

---

## 🧩 インストール手順（Windows）

### ✅ 1. インストーラーをダウンロード

最新の GitHub Release ページから [`pvdp_installer.exe`](https://github.com/trance-mode/primevideo-discord-presence/releases) をダウンロードしてください。

または以下のバッジから直接ダウンロードできます：

[![Download Installer](https://img.shields.io/badge/Download-pvdp__installer.exe-blue?logo=github)](https://github.com/trance-mode/primevideo-discord-presence/releases/latest/download/pvdp_installer.exe)

### ✅ 2. ダブルクリックして実行

以下の処理が自動で行われます：

- `C:\Program Files\primevideo-discord-presence\` に本体を展開
- Chrome拡張をレジストリに登録
- NativeMessaging用マニフェストを登録

> 💡 セキュリティ警告が表示された場合は「詳細情報」→「実行」を選択してください。

---

## 🖥 Chrome 拡張の有効化（初回のみ）

Chrome の仕様により、拡張は初回インストール時に**自動では有効化されません**。以下の手順で有効化してください：

1. `chrome://extensions` にアクセス
2. 「Prime Video Discord Presence」が表示されていることを確認
3. トグルをクリックして「有効」にします ✅

> 🔐 一度有効化すれば、次回以降は自動で有効になります。

---

## 🧹 アンインストール方法 / Uninstall

### 🖱️ GUIアンインストーラーを使う（推奨）

1. GitHub Releases から [`pvdp_uninstaller.exe`](https://github.com/trance-mode/primevideo-discord-presence/releases) をダウンロード
2. **右クリック → 管理者として実行**
3. GUIが表示されるので、表示ログを確認し「Close」ボタンで終了します

実行後、以下が削除されます：
- `C:\Program Files\primevideo-discord-presence`
- `HKEY_CURRENT_USER\Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence`
- `HKEY_CURRENT_USER\Software\Google\Chrome\Extensions\com.pvdp.discord.presence`

---

### 🛠 手動で削除する場合（上級者向け）

```powershell
Remove-Item -Path "C:\Program Files\primevideo-discord-presence" -Recurse -Force
Remove-Item -Path "HKCU:\Software\Google\Chrome\NativeMessagingHosts\com.pvdp.discord.presence" -Force
Remove-Item -Path "HKCU:\Software\Google\Chrome\Extensions\com.pvdp.discord.presence" -Force
```

---

## 📁 ディレクトリ構成（開発者向け）

```
primevideo-discord-presence/
├── extension/                  # Chrome拡張
├── native/                     # Rustネイティブ本体（pvdp, installer, uninstaller）
│   ├── src/main.rs
│   └── src/bin/pvdp_installer.rs
│   └── src/bin/pvdp_uninstaller.rs
├── installer/                  # Native Host manifest
│   └── com.pvdp.discord.presence.json
├── .github/workflows/          # GitHub Actions 定義
├── README.md
└── LICENSE
```

---

## ⚙️ 技術スタック / Tech Stack

- Chrome MV3 Extension（manifest v3）
- Rust + Tokio + [discord-sdk](https://github.com/discord/discord-rs)
- `warp` + WebSocket + `tracing`（リアルタイムログ送信）
- `tray-item`（Windows トレイ常駐）
- `requestAnimationFrame` を使った差分更新
- ログレベル別の色分け表示 / バージョン表示付き UI

---

## 🔄 自動化とCI/CD

このプロジェクトは以下を自動化しています：

- タグ付き push → GitHub Release 自動作成
- `pvdp.exe`, `pvdp_installer.exe`, `pvdp_uninstaller.exe` をビルドして添付
- `manifest.json` / `log.js` / `Cargo.toml` のバージョン同期

---

## 📄 ライセンス

MIT License. See `LICENSE` for details.