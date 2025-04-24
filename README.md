# Prime Video Discord Presence (PVDP)

🎬 Amazon Prime Video の再生状況を Discord のステータスに表示する、**Chrome 拡張機能 + Rust ネイティブアプリ**です。

*A Chrome Extension + Rust Native Host to display your Amazon Prime Video activity as a Discord Rich Presence status.*

![version](https://img.shields.io/github/v/release/trance-mode/primevideo-discord-presence)

---

## 📦 主な機能 / Features

- ✅ Prime Video の再生中／停止中を自動検知
- 🕒 Discord にタイトル・エピソード・残り時間を表示
- 🚀 Rust ネイティブとの高速通信（Chrome Native Messaging）
- 🔌 ログビューア連携（WebSocketでリアルタイムログ表示）
- 🧠 高精度な再生状態判断（UI ボタン検出 + MutationObserver）
- 🎨 ログの色分け / ステータス表示強化（v1.4.0+）

---

## 🧩 インストール手順 / Installation

### ✅ Windows（自動スクリプト）

PowerShell（管理者）で以下を実行：

```powershell
iwr "https://raw.githubusercontent.com/trance-mode/primevideo-discord-presence/main/installer/windows.ps1" | iex
```
📦 自動登録済：拡張機能は Chrome に自動追加されます（必要に応じて手動で有効化してください）。

---

## 🛠 手動インストール手順（開発者向け） / Installation

### 1. Chrome 拡張の読み込み

1. `extension/` を Chrome の「パッケージ化されていない拡張機能」として読み込みます。

### 2. Rust ネイティブアプリのビルド

```sh
cd native/
cargo build --release
```

- ビルド後：`target/release/pvdp(.exe)` が生成されます。

### 3. ネイティブホスト登録（Windows）

```ps1
powershell -ExecutionPolicy Bypass -File installer/windows.ps1
```

- `com.pvdp.discord.presence` をレジストリ登録して、拡張との接続を可能にします。

---

## 📁 ディレクトリ構成 / Project Structure

```
primevideo-discord-presence/
├── extension/                  # Chrome 拡張本体
│   ├── manifest.json
│   ├── background.js
│   ├── content.js
│   ├── popup.html / .js / .css
│   ├── log.html / .js / .css
│   └── logCommon.js
├── native/                     # Rust ネイティブアプリ
│   ├── src/main.rs
│   ├── build.rs
│   └── Cargo.toml
├── installer/                  # Native Host 用スクリプト・マニフェスト
│   ├── windows.ps1 / windows_uninstall.ps1
│   └── resources/com.pvdp.discord.presence.json
├── screenshots/                # スクリーンショット（任意）
├── .github/workflows/rust.yml  # GitHub Actions CI
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

## 🧪 開発支援 / Dev Support

本リポジトリは GitHub Actions による CI を導入しています：

- `rust.yml`: Rust ネイティブアプリのビルド／テスト
- `version-sync.yml`: Cargo.toml のバージョン → PowerShell スクリプトに自動反映
- `release.yml`: タグ付き push 時に GitHub Release を自動作成（pvdp.exe を添付）

---

## 📄 ライセンス / License

MIT License. See `LICENSE` for details.