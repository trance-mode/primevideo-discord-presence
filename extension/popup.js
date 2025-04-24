// === popup.js === (v1.4.0 - WebSocket表示強化 + reconnect-safe + 色付きログ対応)
import { connectLogSocket } from './logCommon.js';

const statusElem = document.getElementById("status");
const logElem    = document.getElementById("log");
const versionElem = document.getElementById("version");

versionElem.textContent = "v1.4.0";

document.getElementById("open-log").addEventListener("click", () => {
  window.open(chrome.runtime.getURL("log.html"), "_blank");
});

// ログ表示＋WebSocket接続維持
connectLogSocket({
  statusElem,
  logElem,
});
