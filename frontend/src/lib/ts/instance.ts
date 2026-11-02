import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from "@tauri-apps/api/event";
import "./instance_card.js";
import { getCurrentWindow } from '@tauri-apps/api/window';
import { confirmDialog } from './confirm_dialog.ts';

export async function loadInstance(container: any) {
     if (!container) return { empty: false};
     container.innerHTML = "";

     try {
          const name: any = await invoke('get_command');
          if (name.length === 0) return { empty: true };

          for (const n of name) {
               const card = document.createElement("div");
               const instanceCard = document.createElement("inst-card");
               instanceCard.setAttribute("name", n);
               card.appendChild(instanceCard);
               container.appendChild(card);
          }
          return { empty: false }
     } catch (err) {
          console.error(err);
          return { empty: false }
     }
}

export async function setup(container: any, onRefresh: any) {
     const refresh = async () => {
          const { empty } = await loadInstance(container);
          onRefresh?.(empty);
     };

     const unlisten = [
          await listen('instance-removed', refresh),
          await listen('closing', refresh),
          await listen('created', refresh),
     ];

     const click = async (e: Event) => {
          const target = e.target;
          if (!(target instanceof Element)) return;
          const btn = target.closest<HTMLButtonElement>('.instance-card');
          if (!btn) return;

          const name = btn.dataset.name;
          if (!name) return;
          const url = `/instance?name=${encodeURIComponent(name)}&title=${encodeURIComponent(name)}`;
          const labelReplace = name.replace(/[^a-zA-Z0-9_-]/g, '-').replace(/-+/g, '-').replace(/^-|-$/g, '');
          const label = `instance-${labelReplace}-${Date.now()}`;

          await invoke('spawn_instance_window', { label, url, name });
     };
     container?.addEventListener('click', click);

     await refresh();
     return () => {
          unlisten.forEach((fn) => fn());
          container?.removeEventListener('click', click);
     };
}

export async function load(loader: any) {     
     await invoke('load_versions', { loader });
     const versions = await invoke('get_version_list', { loader });
     const loaders = (loader === "vanilla" || loader === "forge" || loader === "neoforge") ? [] : await invoke('get_loader_list', { loader, mc: null });
     return { versions , loaders };
}

export const loaderList = (loader: any, mc: any) => invoke('get_loader_list', { loader, mc });

export async function create(name: any, version: any, loader: any, loaderVersion: any) {
     let unlisten: (() => void) | undefined;
     let resolves!: () => void;
     const ready = new Promise<void>((resolve) => { resolves = resolve; });

     try {
          unlisten = await listen("progress-ready", () => resolves());
          await invoke("spawn_progress_window", { url: `/progress?title=${encodeURIComponent("Download Progress")}` });
          await ready;
          await invoke("create_instance", { name, version, loader, loaderVersion });
          await emit('created');
     } finally {
          unlisten?.();
     }
}

export function close(delay: number) {
     setTimeout(() => getCurrentWindow().close(), delay)
}

export const def = (list: any, l: any) => {
     if (l === "forge" || l === "neoforge") {
          return list.find(v => v.ver_type === "release")?.id ?? '';
     }
     return list[0]?.id ?? '';
};

export function getInstance() {
     const params = new URLSearchParams(window.location.search);
     return params.get('name') || 'Unknown';
}

export async function deletes(name: string) {
     const confirm = await confirmDialog(`Delete instance "${name}?`);
     if (!confirm) return false;

     try {
          await invoke('delete_instance', { name });
          return true;
     } catch (e) {
          alert(`Failed to delete instance "${name}": ${e}`);
          return false;
     }
}

export async function launchSave(name: string, world: string) {
     try {
          await invoke('launching', { name, world });
     } catch (e) {
          alert(`Failed to launch "${name}": ${e}`);
     }
}

export async function launchServer(name: string, server: string) {
     try {
          await invoke('launching', { name, world: null, server });
     } catch (e) {
          alert(`Failed to launch "${name}": ${e}`);
     }
}

export async function instanceInfo(name: string) {
     try {
          return await invoke('instance_info', { name });
     } catch (e) {
          alert(`Failed to find instance info for "${name}": ${e}`);
     }
}

export async function launch(name: string) {
     try {
          await invoke('launching', { name });
     } catch (e) {
          alert(`Failed to launch "${name}": ${e}`);
     }
}

export async function open(name: string) {
     try {
          await invoke('open_instance_folder', { name });
     } catch (e) {
          alert(`Failed to open directory for "${name}": ${e}`);
     }
}

export async function showSaves(name: string) {
     return await invoke('list_saves', { name });
}

export async function showServers(name: string) {
     return await invoke('list_servers', { name });
}