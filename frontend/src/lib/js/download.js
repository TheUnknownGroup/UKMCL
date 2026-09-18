import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

export function downloadSetup({ onProgress, onComplete }) {
     const unlisten = [];
     let active = true;
     let timestamp = null;
     let down = 0;
     const samp = [];
     const MAX = 10;
     function format(seconds) {
          if (!isFinite(seconds) || seconds < 0) return '';
          if (seconds < 60) return `${Math.ceil(seconds)}s`;
          const mins = Math.floor(seconds / 60);
          const secess = Math.round(seconds % 60);
          return `${mins}m ${secess}s`;
     }

     listen('progress', (e) => {
          const { downloaded, total, current, files, file_total } = e.payload;
          const pct = total > 0 ? Math.round((downloaded / total) * 100) : 0;
          const now = performance.now();
          let etaTxt = "";

          if (timestamp !== null) {
               const bys = downloaded - down;
               const secs = (now - timestamp) / 1000;

               if (secs > 0 && bys >= 0) {
                    samp.push(bys / secs);
                    if (samp.length > MAX) samp.shift();
                    const avg = samp.reduce((a, b) => a + b, 0) / samp.length;
                    const remain = avg > 0 ? (total - downloaded) / avg : Infinity;
                    etaTxt = format(remain);
               }
          }

          timestamp = now;
          down = downloaded;
          onProgress({ pct, label: `${current} (${files} / ${file_total}) -- ${pct}%`, eta: etaTxt });
     }).then((fn) => (active ? unlisten.push(fn) : fn()));

     listen('complete', () => {
          onComplete();
          setTimeout(() => getCurrentWindow().close(), 800);
     }).then((fn) => (active ? unlisten.push(fn) : fn()));

     return () => {
          active = false;
          for (const fn of unlisten) fn();
     }
}