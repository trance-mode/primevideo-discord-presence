// === log.js === (v1.4.0 - WebSocketログ表示 + レベル別色分け + バージョン表示)
import { connectLogSocket } from './logCommon.js';

const statusElem = document.getElementById("status");
const logElem    = document.getElementById("log");
const versionElem = document.getElementById("version");

versionElem.textContent = "v0.0.0";

connectLogSocket({
  statusElem,
  logElem,
});