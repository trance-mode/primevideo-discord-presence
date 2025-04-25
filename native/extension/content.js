// === content.js ===（THROTTLE_MS重複防止 + bfcache対応 + 再接続済み）

(() => {
  if (window.__pvdp_content_loaded__) return;
  window.__pvdp_content_loaded__ = true;

  // 🔁 bfcache対応：戻る・進む操作でcontext無効時はリロード
  window.addEventListener("pageshow", (event) => {
    if (event.persisted) {
      console.warn("🔁 pageshow (from bfcache) → reloading...");
      window.location.reload();
    }
  });

  const THROTTLE_MS = 3000;
  let lastSent = 0;
  let port = null;
  let lastRemainingSec = null;
  let lastPlayState = null;
  let lastVideoRef = null;
  let lastVideoSrc = null;
  let lastTitle = null;
  let lastEpisode = null;
  let wasPlayerActive = false;

  function openPort() {
    if (port) return;
    try {
      if (typeof chrome.runtime?.connect === "function") {
        port = chrome.runtime.connect({ name: "pvdp-content" });
        console.info("🔌 Port connected (content → background)");
        port.onDisconnect.addListener(() => {
          console.warn("⚠️ Port disconnected");
          port = null;
        });
      }
    } catch (e) {
      console.debug("❌ Failed to connect to background:", e);
      port = null;
    }
  }

  function send(obj) {
    openPort();
    try {
      port?.postMessage(obj);
    } catch (e) {
      console.debug("❌ postMessage failed:", e);
      port = null;
    }
  }

  function hms(sec) {
    if (!Number.isFinite(sec)) return "00:00";
    const s = String(Math.floor(sec % 60)).padStart(2, "0");
    const m = String(Math.floor(sec / 60) % 60).padStart(2, "0");
    const h = Math.floor(sec / 3600);
    return h ? `${h}:${m}:${s}` : `${m}:${s}`;
  }

  function toSeconds(str) {
    const parts = str.split(":").map(Number);
    return parts.length === 3
      ? parts[0] * 3600 + parts[1] * 60 + parts[2]
      : parts[0] * 60 + parts[1];
  }

  function $txt(sel) {
    return document.querySelector(sel)?.textContent.trim() || "";
  }

  function isPlayerActive() {
    return !!document.querySelector("#dv-web-player video");
  }

  function isPlaying() {
    const btn = document.querySelector(
      "#dv-web-player > div > div.webPlayerSDKUiContainer > div > div > div > div > div.atvwebplayersdk-overlays-container.fpqiyer.f1sp4gm7 > div.ffszj3z.f8hspre.f1icw8u > div.f1aiijcp.fw80uk2 > div:nth-child(2) > div > button"
    );

    if (!btn) {
      console.debug("🔍 isPlaying: ボタンが見つからない");
      return false;
    }

    const aria = btn.getAttribute("aria-label")?.trim();
    const text = btn.textContent?.trim();
    console.log("🔍 isPlaying: aria-label =", aria, "| text =", text);

    if (aria === "再生" || aria === "Play") {
      console.log("⏹ 状態: 停止中（Playボタン表示中）");
      return false;
    }

    if (aria === "一時停止" || aria === "Pause") {
      console.log("▶ 状態: 再生中（Pauseボタン表示中）");
      return true;
    }

    console.warn("❓ isPlaying: 状態不明 - aria-label =", aria);
    return false;
  }

  function getTimes() {
    const timeNode = document.querySelector("div.fage5o5.f1mic5r1 > div");
    const remainingSpan = timeNode?.querySelector("span");
    const currentRaw = [...timeNode?.childNodes || []]
      .find((n) => n.nodeType === Node.TEXT_NODE && n.textContent.trim())
      ?.textContent.trim();
    const remainingRaw = remainingSpan?.textContent.trim().replace("/", "");

    if (!currentRaw || !remainingRaw)
      return { current: "00:00", total: "00:00" };

    const cur = toSeconds(currentRaw);
    const rem = toSeconds(remainingRaw);
    return { current: currentRaw, total: hms(cur + rem) };
  }

  function buildPresence(playingNow) {
    if (!isPlayerActive()) return null;
    const times = getTimes();
    if (times.current === "00:00" && times.total === "00:00") return null;

    const title =
      $txt("h1.atvwebplayersdk-title-text") ||
      document.title.replace(/ - Prime Video$/, "");
    const episode = $txt("h2.atvwebplayersdk-subtitle-text") || "No episode";
    const remainingSec = toSeconds(times.total) - toSeconds(times.current);

    if (
      lastRemainingSec !== null &&
      Math.abs(remainingSec - lastRemainingSec) < 10
    )
      return null;

    lastRemainingSec = remainingSec;

    if (!playingNow) {
      return {
        message_type: 4,
        title,
        episodes: "⏹ 停止中",
        current_time: "00:00",
        total_duration: "00:00",
      };
    }

    return {
      message_type: 3,
      title,
      episodes: `▶ 再生中 | ${episode} | 残り ${hms(remainingSec)}`,
      current_time: times.current,
      total_duration: times.total,
    };
  }

  new MutationObserver(() => {
    const video = document.querySelector("video");
    if (!video) return;

    const changed =
      video !== lastVideoRef ||
      video.src !== lastVideoSrc ||
      $txt("h1.atvwebplayersdk-title-text") !== lastTitle ||
      $txt("h2.atvwebplayersdk-subtitle-text") !== lastEpisode;

    if (changed) {
      lastVideoRef = video;
      lastVideoSrc = video.src;
      lastTitle = $txt("h1.atvwebplayersdk-title-text");
      lastEpisode = $txt("h2.atvwebplayersdk-subtitle-text");
      lastSent = 0;
      lastRemainingSec = null;
      lastPlayState = null;
    }
  }).observe(document, { childList: true, subtree: true });

  setInterval(() => {
    const now = Date.now();
    const active = isPlayerActive();

    if (!active) {
      if (wasPlayerActive) {
        send({
          message_type: 4,
          title: "Prime Video",
          episodes: "⏹ 停止中",
          current_time: "00:00",
          total_duration: "00:00",
        });
        wasPlayerActive = false;
      }
      return;
    }

    wasPlayerActive = true;

    const playingNow = isPlaying();

    if (playingNow !== lastPlayState) {
      lastSent = now - THROTTLE_MS;
    }

    if (now - lastSent < THROTTLE_MS && playingNow === lastPlayState) return;

    lastSent = now;
    lastPlayState = playingNow;

    const presence = buildPresence(playingNow);
    presence && send(presence);
  }, 1000);

  setInterval(() => {
    if (!port) openPort();
    if (port) {
      try {
        port.postMessage({ message_type: 0 });
      } catch (e) {
        console.debug("❌ Failed to send keep-alive ping:", e);
        port = null;
      }
    }
  }, 15000);
})();
