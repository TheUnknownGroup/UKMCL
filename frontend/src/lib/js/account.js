import { invoke } from "@tauri-apps/api/core";

export async function acc() {
     return await invoke("acc_info");
}