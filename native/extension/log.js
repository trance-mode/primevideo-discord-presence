// === log.js === (v1.4.0 - WebSocketログ表示 + レベル別色分け + バージョン表示)
import { connectLogSocket } from './logCommon.js';

const statusElem   = document.getElementById("status");
const logElem      = document.getElementById("log");
const versionElem  = document.getElementById("version");

// 🔄 バージョンを manifest.json から自動取得
versionElem.textContent = `v${chrome.runtime.getManifest().version}`;

connectLogSocket({
  statusElem,
  logElem,
});