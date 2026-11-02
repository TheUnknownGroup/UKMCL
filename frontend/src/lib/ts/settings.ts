import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

export async function java_minimum(): Promise<number> {
     return await invoke("get_java_minimum");
}

export async function java_maximum(): Promise<number> {
     return await invoke("get_java_maximum");
}

export async function set_ram_usage(args: Record<string, unknown>): Promise<void> {
     await invoke("ram", args);
}

let resolver: ((result: unknown) => void) | null = null;

export async function auth() {
     await invoke('microsoft_auth');
}

export function resolveAuth(result: unknown) {
     resolver?.(result);
     resolver = null;
}

export function createCount(expires: any, onT: any, onE: any) {
     let id = null;
     const stop = () => { if (id !== null) { clearInterval(id); id = null; } };
     const tick = () => {
          const now = Math.floor(Date.now() / 1000)
          const remaining = Math.max(0, expires - now);
          onT(remaining);
          if (remaining <= 0) { stop(); onE?.(); }
     };
     tick();
     id = setInterval(tick, 1000);
     return stop;
}

export function format(secs: number): string {
     const m = Math.floor(secs / 60);
     const s = secs % 60;
     return `${m}:${s.toString().padStart(2, '0')}`;
}

export async function copy(text: string): Promise<boolean> {
     try {
          await writeText(text);
          return true;
     } catch (e) {
          console.error("Failed to copy to clipboard: ", e);
          return false;
     }
}

export async function cancel(): Promise<void> {
     if (resolver) return;
     await invoke("cancel");
     setTimeout(() => {}, 500);
     return emit('cancels');
}

export async function account() {
     return await invoke("account_info");
}

export async function microsoft() {
     try {
          await auth();
          setTimeout(() => { }, 500);
          await emit("authed");
     } catch (e) {
          console.error("Microsoft auth failed: ", e);
     }
}

export async function signOut() {
     await invoke('sign_out');
     setTimeout(() => {}, 500);
     return emit('authed');
}