// === background.js === (ログ強化版)
let native = null;
let wsClients = [];

function openNative() {
  if (native) return;
  native = chrome.runtime.connectNative("com.pvdp.discord.presence");
  console.info("🔌 Native port connected");
  native.onDisconnect.addListener(() => {
    console.warn("⚠️ Native disconnected", chrome.runtime.lastError?.message);
    native = null;
  });
}
openNative();

chrome.runtime.onConnect.addListener((port) => {
  if (port.name === "pvdp-content") {
    console.info("🔁 Content port connected");
    port.onMessage.addListener((msg) => {
      console.debug("📨 Forwarding to native:", JSON.stringify(msg));
      openNative();
      native?.postMessage(msg);
    });
    port.onDisconnect.addListener(() => {
      console.warn("⚠️ Content port disconnected");
      openNative();
      native?.postMessage({
        message_type: 4,
        title: "Prime Video",
        episodes: "⏹ 停止中",
        current_time: "00:00",
        total_duration: "00:00"
      });
    });
  } else if (port.name === "pvdp-log") {
    console.info("📡 Log view client connected");
    wsClients.push(port);
    port.onDisconnect.addListener(() => {
      wsClients = wsClients.filter(p => p !== port);
    });
  }
});

setInterval(() => {
  openNative();
  native?.postMessage({ message_type: 0 });
}, 25000);

chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === "complete" && tab.url?.includes("/gp/video/")) {
    if (!chrome.scripting) {
      console.error("❌ chrome.scripting is undefined");
      return;
    }
    chrome.scripting.executeScript({
      target: { tabId },
      files: ["content.js"]
    }).then(() => {
      console.info("✅ Reinjected content.js");
    }).catch((err) => {
      console.warn("❌ Failed to inject content.js:", err);
    });
  }
});