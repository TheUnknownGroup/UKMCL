import { invoke } from '@tauri-apps/api/core';
import { emit } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

export async function load() {
     await invoke('load_versions');
     return await invoke('get');
}

export async function create(instName, ver) {
     await invoke('create_command', { instName: instName, ver: ver })
     await emit('created');
}

export function close(delay) {
     setTimeout(() => getCurrentWindow().close(), delay)
}