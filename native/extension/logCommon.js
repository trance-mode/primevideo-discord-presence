// === logCommon.js === (v1.4.0 - WebSocketログ表示 + ログレベル別色分け + 自動再接続対応)
export function connectLogSocket({ statusElem, logElem }) {
    let ws = null;
    let retryCount = 0;
  
    function appendLog(text) {
      const time = new Date().toLocaleString(undefined, {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
        hour12: false
      });
  
      const div = document.createElement("div");
      div.textContent = `[${time}] ${text}`;
      div.className = getLogLevelClass(text);
      const shouldScroll = isScrolledToBottom();
      logElem.appendChild(div);
  
      if (logElem.childNodes.length > 1000) {
        logElem.removeChild(logElem.firstChild);
      }
  
      if (shouldScroll) {
        logElem.parentElement.scrollTop = logElem.parentElement.scrollHeight;
      }
    }
  
    function getLogLevelClass(text) {
      const msg = text.toLowerCase();
      if (msg.includes("error") || msg.includes("❌")) return "log-error";
      if (msg.includes("warn") || msg.includes("⚠️")) return "log-warn";
      if (msg.includes("debug") || msg.includes("📬") || msg.includes("📨")) return "log-debug";
      return "log-info";
    }
  
    function isScrolledToBottom() {
      const container = logElem.parentElement;
      return container.scrollHeight - container.scrollTop <= container.clientHeight + 10;
    }
  
    function updateStatus(text, className) {
      statusElem.textContent = text;
      statusElem.className = "status-indicator " + className;
    }
  
    function connectWs() {
      if (ws) return;
  
      const url = "ws://127.0.0.1:3012/ws";
      appendLog(`🔄 Connecting to ${url}…`);
      updateStatus("🔄 Connecting…", "status-warn");
  
      try {
        ws = new WebSocket(url);
      } catch (e) {
        appendLog(`❗ Failed to open WebSocket: ${e.message}`);
        scheduleReconnect();
        return;
      }
  
      ws.addEventListener("open", () => {
        retryCount = 0;
        updateStatus("✅ WebSocket connected", "status-ok");
        appendLog("🔌 WebSocket connected");
      });
  
      ws.addEventListener("message", (e) => {
        appendLog(e.data);
      });
  
      ws.addEventListener("error", (e) => {
        console.warn("WebSocket error:", e);
        appendLog(`❗ WebSocket error: ${e.message || e}`);
        handleClose();
      });
  
      ws.addEventListener("close", handleClose);
    }
  
    function handleClose() {
      if (ws) {
        ws.removeEventListener("close", handleClose);
        ws.close();
        ws = null;
      }
      updateStatus("⚠️ WebSocket disconnected", "status-error");
      appendLog("⚠️ WebSocket disconnected");
      scheduleReconnect();
    }
  
    function scheduleReconnect() {
      retryCount++;
      const delay = Math.min(30000, 3000 * retryCount);
      appendLog(`⏳ Reconnecting in ${delay / 1000}s…`);
      setTimeout(connectWs, delay);
    }
  
    // 初回接続
    connectWs();
  }
  