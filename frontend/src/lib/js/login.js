import { invoke } from "@tauri-apps/api/core";

let resolver = null;

export function auth() {
     return new Promise((resolve) => {
          resolver = { resolve };
          invoke("microsoft_auth");
     });
}

export function resolveAuth(result) {
     if (!resolver) return;
     resolver.resolve(result);
     resolver = null;
}

export function createCount(expires, onT, onE) {
     let id = null;
     const stop = () => { if (id !== null) { clearInterval(id); id = null; } };
     const tick = () => {
          const now = Math.floor(Date.now() / 1000);
          const remaining = Math.max(0, expires - now);
          onT(remaining);
          if (remaining <= 0) { stop(); onExpire?.(); }
     };
     tick();
     id = setInterval(tick, 1000);
     return stop;
}

export function format(secs) {
     const m = Math.floor(secs / 60);
     const s = secs % 60;
     return `${m}:${s.toString().padStart(2, '0')}`;
}

export async function copys(text) {
     try {
          await navigator.clipboard.writeText(text);
          return true;
     } catch (e) {
          console.error(e);
          return false;
     }
}