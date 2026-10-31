import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from "@tauri-apps/api/event";
import "./instance_card.js";
import { getCurrentWindow } from '@tauri-apps/api/window';

export async function loadInstance(container: any) {
     if (!container) return { empty: false};
     container.innerHTML = "";

     try {
          const name: any = await invoke('get_command');
          if (name.length === 0) return { empty: true };

          for (const n of name) {
               const card = document.createElement("div");
               card.innerHTML = `<inst-card name="${n}"></inst-card>`;
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

     container?.addEventListener('click', async (e) => {
          const btn = e.target.closest('.instance-card');
          if (!btn) return;

          const name = btn.dataset.name;
          const url = `/instance?name=${encodeURIComponent(name)}`;
          const labelReplace = name.replace(/[^a-zA-Z0-9_-]/g, '-').replace(/-+/g, '-').replace(/^-|-$/g, '');
          const label = `instance-${labelReplace}-${Date.now()}`;

          await invoke('spawn_instance_window', { label, url, name });
     });

     await refresh();
     return () => unlisten.forEach((fn) => fn());
}

export async function load(loader: any) {     
     await invoke('load_versions', { loader: loader });
     const versions = await invoke('get_version_list', { loader: loader });
     const loaders = (loader === "vanilla" || loader === "forge" || loader === "neoforge") ? [] : await invoke('get_loader_list', { loader: loader, mc: null });
     return { versions , loaders };
}

export const loaderList = (loader: any, mc: any) => invoke('get_loader_list', { loader, mc });

export async function create(name: any, version: any, loader: any, loaderVersion: any) {
     await invoke('create_instance', { name, version, loader, loaderVersion });
     await emit('created');
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