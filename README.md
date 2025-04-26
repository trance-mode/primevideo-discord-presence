# Prime Video Discord Presence (PVDP)

🎬 Amazon Prime Video の再生状況を Discord のステータスに表示する、**Chrome拡張機能 + Rustネイティブアプリ**です。

*A Chrome Extension + Rust Native Host to display your Amazon Prime Video activity as a Discord Rich Presence status.*

<img src="https://github.com/user-attachments/assets/54e97a60-f597-4760-8b43-3ae50992528a" width="500" />

![version](https://img.shields.io/github/v/release/trance-mode/primevideo-discord-presence)
[![Download Installer](https://img.shields.io/badge/Download-pvdp__installer.exe-blue?logo=github)](https://github.com/trance-mode/primevideo-discord-presence/releases/latest/download/pvdp_installer.exe)
[![Download Uninstaller](https://img.shields.io/badge/Download-pvdp__uninstaller.exe-blue?logo=github)](https://github.com/trance-mode/primevideo-discord-presence/releases/latest/download/pvdp_uninstaller.exe)

---

## 📦 主な機能 / Features

- ✅ Prime Video の再生中／停止中を自動検知
- 🕒 Discord にタイトル・エピソード・残り時間を表示
- 🚀 Rustネイティブとの高速通信 (Chrome Native Messaging)
- 🔌 リアルタイムログビューア連携 (WebSocket)
- 🧠 高精度な再生状態判断 (UIボタン検出 + MutationObserver)
- 🎨 ログの色分け / ステータス表示強化 (v1.4.0+)

---

## 🧩 インストール手順（Windows）

### ✅ 1. インストーラーをダウンロードして実行（管理者として）

[![Download Installer](https://img.shields.io/badge/Download-pvdp__installer.exe-blue?logo=github)](https://github.com/trance-mode/primevideo-discord-presence/releases/latest/download/pvdp_installer.exe)

実行すると、以下が自動で行われます：

- `C:\Program Files\primevideo-discord-presence\` に本体ファイルを展開
- NativeMessaging用マニフェストを生成・レジストリ登録

> 💡 インストール完了後、Chromeと拡張機能フォルダを開くボタンが表示されます。

### ✅ 2. extensionフォルダをChrome拡張機能に追加

- C:\Program Files\primevideo-discord-presence\にextensionフォルダが追加されます。
- 詳しくは下のChrome拡張機能を手動で追加する方法をご覧ください。

> 💡 追加後、プライム動画を再生するとDiscordに表示されます。

---

### 📢 注意

当初予定していた **「Chrome Web Store」登録は行わず、ローカルに拡張機能を手動追加する方式** に変更しました。

- Chrome拡張のストア登録には有料（登録料）が必要だったため
- 現段階では、**ローカル手動追加**方式に切り替えています

> ❗ 将来的には、情勢や需要を見て「Chrome Web Store登録」を検討する可能性もあります。

---

## 🖥 Chrome拡張機能を手動で追加する方法

1. Chromeを開き、アドレスバーに「`chrome://extensions`」と入力してアクセス
2. 右上の「開発者モード」をONにする
3. 「パッケージ化されていない拡張機能を読み込む」をクリック
4. インストーラーが開いた `extension` フォルダを選択

> 🔐 一度読み込めば、次回以降は自動で有効になります。

---

## 🧹 アンインストール方法 / Uninstall

### 🖱️ GUIアンインストーラーを使う（推奨）

[![Download Uninstaller](https://img.shields.io/badge/Download-pvdp__uninstaller.exe-blue?logo=github)](https://github.com/trance-mode/primevideo-discord-presence/releases/latest/download/pvdp_uninstaller.exe)

---

## ⚙ 開発者向け

1. ポップアップに固定した拡張機能を押すことでログが確認できます。
   <img src="https://github.com/user-attachments/assets/de21e46a-fe7a-4213-b050-cf8892a2e5ed" width="250" />
2. ログを別ウィンドウで見るボタンを押すことで別ウィンドウで確認することも出来ます。
   <img src="https://github.com/user-attachments/assets/e76c1068-6757-4d98-a5c9-6a5012938735" width="500" />

---

## 📁 ディレクトリ構成

```
primevideo-discord-presence/
├── extension/
├── native/
│   ├── src/main.rs
│   ├── src/bin/pvdp_installer.rs
│   └── src/bin/pvdp_uninstaller.rs
├── .github/workflows/
├── README.md
└── LICENSE
```

---

## ⚙️ 技術スタック

- Chrome Manifest V3 Extension
- Rust + Tokio + discord-sdk
- warp + WebSocket + tracing
- tray-item
- requestAnimationFrame

---

## 🔄 自動化とCI/CD

- GitHub Release自動作成
- `pvdp.exe`, `pvdp_installer.exe`, `pvdp_uninstaller.exe` ビルド＆添付
- バージョン情報同期 (manifest.json, log.js, Cargo.toml)

---

## 📄 ライセンス

MIT License. See `LICENSE` for details。
