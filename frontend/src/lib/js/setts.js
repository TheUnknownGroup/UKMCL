import { invoke } from "@tauri-apps/api/core";

export async function spawn() {
     await invoke("spawn_off");
}

export async function mins() {
     return await invoke("get_java_min")
}

export async function maxs() {
     return await invoke("get_java_max")
}

export async function rams(args) {
     await invoke("ram", args);
}