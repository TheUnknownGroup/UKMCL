import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { confirmDialog } from './confirmDialog';

export function getInst() {
     const params = new URLSearchParams(window.location.search);
     return params.get('name') || 'Unknown';
}

export async function deletes(name) {
     const confirmed = await confirmDialog(`Delete instance "${name}"?`);
     if (!confirmed) return false;

     try {
          await invoke('delete_command', { instName: name });
          setTimeout(() => getCurrentWindow().close(), 600);
          return true;
     } catch (err) {
          alert(`Failed to delete instance "${name}": ${err}`);
          return false;
     }
}

export async function launch(name) {
     try {
          await invoke('launch_command', { instName: name });
     } catch (err) {
          alert(`Failed to launch "${name}": ${err}`);
     }
}

export async function launchSave(name, world) {
     try {
          await invoke('launch_command', { instName: name, world: world });
     } catch (err) {
          alert(`Failed to launch "${name}": ${err}`);
     }
}

export async function launchServ(name, server) {
     try {
          await invoke('launch_command', { instName: name, world: null, server: server });
     } catch (err) {
          alert(`Failed to launch "${name}": ${err}`);
     }
}

export async function edit(name) {
     try {
          await invoke('open', { name: name });
     } catch (err) {
          alert(`Failed to open directory for "${name}": ${err}`);
     }
}

export async function instInfo(name) {
     try {
          return await invoke('inst_info', { name: name });
     } catch (e) {
          alert(`Failed to find instance info for: "${name}: ${e}"`);
     }
}

export async function mods() {
     try {
          await invoke('spawn_modss');
     } catch (e) {
          alert(`Failed to find mods window: "${e}"`);
     }
}

export async function showSaves(inst) {
     return await invoke('list_saves', { instName: inst });
}

export async function showServs(inst) {
     return await invoke('list_servs', { instName: inst });
}