import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

const fill = document.getElementById("download-fill");
const label = document.getElementById("download-label");
const eta = document.getElementById("eta");

let timestamp = null;
let down = 0;
const samp = [];
const MAX = 10;

function format(seconds) {
     if (!isFinite(seconds) || seconds < 0) return '';
     if (seconds < 60) return `${Math.ceil(seconds)}s`;
     const mins = Math.floor(seconds / 60);
     const secs = Math.round(seconds % 60);
     return `${mins}m ${secs}s`;
}

await listen('progress', (e) => {
     const { downloaded, total, current, files, file_total } = e.payload;
     const pct = total > 0 ? Math.round((downloaded / total) * 100) : 0;

     const now = performance.now();
     
     fill.style.width = `${pct}%`;
     label.textContent = `${current} (${files} / ${file_total}) -- ${pct}%`;

     if (timestamp !== null) {
          const bys = downloaded - down;
          const secs = (now - timestamp) / 1000;

          if (secs > 0 && bys >= 0) {
               const spee = bys / secs;
               samp.push(spee);
               if (samp.length > MAX) samp.shift();
               const avg = samp.reduce((a, b) => a + b, 0) / samp.length;
               const remainbys = total - downloaded;
               const remainsecs = avg > 0 ? remainbys / avg : Infinity;

               eta.textContent = format(remainsecs);
          }
     }

     timestamp = now;
     down = downloaded;
});

await listen('complete', () => {
     fill.style.width = '100%';
     eta.textContent = '';
     setTimeout(() => getCurrentWindow().close(), 800); 
});