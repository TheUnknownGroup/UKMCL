import { invoke } from "@tauri-apps/api/core";

export async function conf() {
     await invoke('write');
     return true;
}