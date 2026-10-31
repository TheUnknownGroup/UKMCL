<script lang="ts">
     import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
     import { page } from '$app/state';
     import { onMount } from "svelte";
     import { scale } from "svelte/transition";
     let title = $derived(page.url.searchParams.get('title') ?? '');
     const win = getCurrentWindow();
     let maxed = $state(false);
     onMount(() => {
          if (win.label == "main") {
               win.setMinSize(new LogicalSize(1112, 636));
               let unlisten: (() => void) | undefined;
               const sync = async () => {
                    maxed = await win.isMaximized();
               };
               sync();
               win.onResized(sync).then((fn) => (unlisten = fn));
               return () => unlisten?.();
          }
     });
     let baseClasses = "w-[45px] h-[30px] bg-transparent border-0 text-inherit cursor-pointer select-none leading-0 flex items-center justify-center";
     const mini = () => win.minimize();
     const maxi = async () => await win.isResizable() ? win.toggleMaximize() : console.log("Not toggleable");
     const close = () => win.close();
     
     const ITEMS = $derived([
          { id: 'Minimize', classes: baseClasses, extra: 'hover:bg-white/15', src: "/assets/images/minimize.svg", wi: "20", click: mini },
          { id: 'Maximize', classes: baseClasses, extra: 'hover:bg-white/15', src: maxed ? '/assets/images/fullscreen-exit.svg' : '/assets/images/fullscreen.svg', wi: "15", click: maxi },
          { id: 'Close', classes: baseClasses, extra: 'hover:bg-vanilla-custard-300', src: "/assets/images/close.svg", wi: "20", click: close },
     ]);
</script>

<div class="fixed text-emerald-50 w-full bg-rich-cerulean-850 h-[30px] grid grid-cols-[1fr_auto_1fr] items-center select-none pb-[30px]" data-tauri-drag-region>
     <div data-tauri-drag-region></div>
     <span class="text-[16px] leading-7" data-tauri-drag-region>{title ? `UKMCL | ${title}` : 'UKMCL'}</span>
     <div class="flex justify-end" data-tauri-drag-region>
          {#each ITEMS as i}
               <button onclick={i.click} class="{i.classes} {i.extra}"><img in:scale={{duration: 300, start: 0.6 }} src={i.src} alt={i.id} width={i.wi} loading="lazy" decoding="async"></button>
          {/each}
     </div>
</div>