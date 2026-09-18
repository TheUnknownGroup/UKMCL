import { invoke } from '@tauri-apps/api/core';
import { listen } from "@tauri-apps/api/event";
import "./instCard.js";

export async function loadInstance(container) {
     if (!container) return { empty: false};
     container.innerHTML = "";

     try {
          const names = await invoke('get_command');
          if (names.length === 0) return { empty: true };

          for (const name of names) {
               const card = document.createElement("div");
               card.innerHTML = `<inst-card name="${name}"></inst-card>`;
               container.appendChild(card);
          }
          return { empty: false }
     } catch (err) {
          console.error(err);
          return { empty: false }
     }
}

export async function setup(container, onRefresh) {
     const refresh = async () => {
          const { empty } = await loadInstance(container);
          onRefresh?.(empty);
     };

     const unlisten = [
          await listen('instance-removed', refresh),
          await listen('closing', refresh),
          await listen('created', refresh),
     ]

     container?.addEventListener('click', async (e) => {
          const btn = e.target.closest('.instance-card');
          if (!btn) return;

          const name = btn.dataset.name;
          const url = `/instance?name=${encodeURIComponent(name)}`;
          const lab = name.replace(/[^a-zA-Z0-9_-]/g, '-').replace(/-+/g, '-').replace(/^-|-$/g, '');
          const label = `instance-${lab}-${Date.now()}`;

          await invoke('spawn_window_2', { label, url, name });
     });

     await refresh();
     return () => unlisten.forEach((fn) => fn());
}