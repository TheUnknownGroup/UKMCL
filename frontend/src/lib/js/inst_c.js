import { invoke } from '@tauri-apps/api/core';
import { emit } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

export async function load(loader) {     
     await invoke('load_versions', { loader: loader });
     const versions = await invoke('get', { loader: loader });
     const loaders = loader === "vanilla" ? [] : await invoke('get_loader', { loader: loader });
     return { versions, loaders };
}

export async function create(instName, ver, loader, loader_version) {
     await invoke('create_command', { instName: instName, ver: ver, loader: loader, loaderVer: loader_version });
     await emit('created');
}

export function close(delay) {
     setTimeout(() => getCurrentWindow().close(), delay)
}