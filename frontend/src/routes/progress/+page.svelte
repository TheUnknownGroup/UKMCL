<script lang="ts">
     import { listen } from "@tauri-apps/api/event";
     import { getCurrentWindow } from "@tauri-apps/api/window";
     import { onMount } from "svelte";
     type ProgressPayload = {
          phase: "assets" | "libraries" | string;
          downloaded: number;
          total: number;
          current: string;
          files: number;
          fileTotal: number;
     }

     let phase = $state('');
     let current = $state('');
     let downloaded = $state(0);
     let total = $state(0);
     let files = $state(0);
     let fileTotal = $state(0);
     let pert = $state(0);
     let eta = $state('');
     let status = $state<"working" | "complete" | "error">("working");

     let lastBytes = 0;
     let lastFile = 0;
     let times = 0;
     let phases = "";
     let smoothed = 0;
     let fileRate = 0;

     function formatBytes(value: number) {
          if (!Number.isFinite(value) || value <= 0) return "0B";
          const units = ["B", "Kb", "Mb", "Gb"];
          let amount = value;
          let index = 0;
          while (amount >= 1024 && index < units.length - 1) {
               amount /= 1024;
               index++;
          }
          return `${amount.toFixed(index === 0 ? 0 : 1)} ${units[index]}`;
     }

     function formatTime(seconds: number) {
          if (!Number.isFinite(seconds) || seconds < 0) return "Calculating...";
          if (seconds < 60) return `${Math.ceil(seconds)} sec(s) remaining`;
          const minutes = Math.floor(seconds / 60);
          const rest = Math.ceil(seconds % 60);
          return rest ? `${minutes}m:${rest}s remaining` : `${minutes}m remaining`;
     }

     function update(payload: ProgressPayload) {
          phase = payload.phase === "assets" ? "Downloading Minecraft assets.." : payload.phase === "libraries" ? "Downloading libraries.." : payload.phase;
          current = payload.current || "Working...";
          downloaded = payload.downloaded ?? 0;
          total = payload.total ?? 0;
          files = payload.files ?? 0;
          fileTotal = payload.fileTotal ?? 0;

          if (total > 0) {
               pert = Math.min(100, Math.floor((downloaded / total) * 100));
          } else if (fileTotal > 0) {
               pert = Math.min(100, Math.floor((files / fileTotal) * 100));
          } else {
               pert = 100;
          }

          const now = Date.now();
          if (payload.phase != phases) {
               lastBytes = downloaded;
               lastFile = files;
               times = now;
               phases = payload.phase;
               smoothed = 0;
               fileRate = 0;
          } else if (times > 0 && now > times) {
               const elasped = (now - times) / 1000;
               if (total > 0 && downloaded >= lastBytes) {
                    const instant = (downloaded - lastBytes) / elasped;
                    if (instant > 0) smoothed = smoothed === 0 ? instant : smoothed * 0.7 + instant * 0.3;
               }
               if (fileTotal > 0 && files >= lastFile) {
                    const instant = (files - lastFile) / elasped;
                    if (instant > 0) fileRate = fileRate === 0 ? instant : fileRate * 0.7 + instant * 0.3;
               }
          }

          lastBytes = downloaded;
          lastFile = files;
          times = now
          eta = total > 0 && smoothed > 0 ? formatTime((total - downloaded) / smoothed) : fileTotal > 0 ? `${files} of ${fileTotal} files` : "Finishing up...";
     }

     onMount(() => {
          let disposed = false;
          let unlisteners: Array<() => void> = [];
          (async () => {
               const progress = await listen<ProgressPayload>("progress", (event) => update(event.payload));
               const complete = await listen("progress-complete", () => {
                    status = "complete";
                    phase = "Installation Complete";
                    current = "Your Minecraft instance is ready.";
                    pert = 100;
                    eta = "Complete";
                    setTimeout(() => getCurrentWindow().close(), 1800);
               });
               const error = await listen<string>("progress-error", (event) => {
                    status = "error";
                    phase = "Installation failed";
                    current = "Installation could not be complete";
                    eta = "Stopped";
               });

               if (disposed) {
                    progress(); complete(); error();
                    return;
               }

               unlisteners = [progress, complete, error];
               const { emit } = await import("@tauri-apps/api/event");
               await emit("progress-ready");
          })();

          return () => {
               disposed = true;
               unlisteners.forEach((unlisten) => unlisten());
          };
     });

     async function closeWindow() {
          await getCurrentWindow().close();
     }
</script>
<div>
     <div class="fixed inset-0 flex h-6.25 top-[50%] border border-black mx-1.25 bg-white/10">
          <div class="h-full w-[0%] bg-[#4caf50] transition-[width] duration-300 ease-in-out" style:width="{pert}%"></div>
          <div class="fixed">
               <h1 class="translate-y-[-270%]">Current Phase: {phase}</h1>
          </div>
          <div class="flex justify-start">
               <span class="absolute left-2 text-[12px] leading-4 text-white whitespace-nowrap flex items-center justify-center translate-y-[-130%] border-white border bg-black/30 px-1.25 pt-1.25 pb-0.75 select-none">{current}</span>
               <span class="text-white text-[15px] translate-y-[13%] right-[48.8%] absolute select-none leading">{pert}%</span>
               <span class="absolute translate-y-[130%] right-0 leading-5 text-[12px] text-[#ccc] select-none">{eta}</span>
               <span class="translate-y-[170%] absolute left-1 flex justify-between text-[11px] text-white/45">{files} / {fileTotal} files</span>
               <span class="absolute translate-y-[200%] right-0 leading-5 text-[12px] text-[#ccc] select-none">{total > 0 ? `${formatBytes(downloaded)} / ${formatBytes(total)}` : ""}</span>
          </div>
     </div>
     {#if status === "error"}
          <div class="flex justify-end translate-y-[690%] translate-x-[-1.3%]">
               <button class="rounded-md bg-white/15 hover:bg-white/25 px-3 pt-1.25 pb-0.5 text-sm cursor-pointer" onclick={closeWindow}>Close</button>
          </div>
     {/if}
</div>
