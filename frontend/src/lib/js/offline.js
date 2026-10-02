import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

export async function account(name) {
     await invoke('offline_account', { name: name });
}

export function close(delay) {
     setTimeout(() => getCurrentWindow().close(), delay)
}