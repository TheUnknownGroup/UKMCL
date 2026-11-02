import { invoke } from "@tauri-apps/api/core";

export async function spawn_offline_window(): Promise<void> {
     await invoke("spawn_offline");
}