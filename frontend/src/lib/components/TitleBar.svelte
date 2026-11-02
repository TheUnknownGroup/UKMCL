<script lang="ts">
     import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
     import { page } from '$app/state';
     import { onMount } from "svelte";
     import { initTheme, theme, toggle } from "#lib/theme.svelte.ts";
     const win = getCurrentWindow();
     let title = $derived(page.url.searchParams.get('title') ?? '');
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
          initTheme();
     });
     let baseClasses = "w-[45px] h-[30px] bg-transparent border-0 text-inherit cursor-pointer select-none leading-0 flex items-center justify-center";
     const mini = () => win.minimize();
     const maxi = async () => await win.isResizable() ? win.toggleMaximize() : console.log("Not toggleable");
     const close = () => win.close();
     
     const ITEMS = $derived([
          { id: '', classes: baseClasses, extra: 'hover:bg-white/5', i: '', wi: '', click: toggle, label: "Toggle theme"},
          { id: 'Minimize', classes: baseClasses, extra: 'hover:bg-white/5', i: 'bi-dash-lg', wi: "20", click: mini },
          { id: 'Maximize', classes: baseClasses, extra: 'hover:bg-white/5', i: maxed ? 'bi-fullscreen-exit' : 'bi-fullscreen', wi: "15", click: maxi },
          { id: 'Close', classes: baseClasses, extra: 'hover:bg-vanilla-custard-300/50', i: "bi-x", wi: "20", click: close},
     ]);
</script>

<div class="fixed text-white w-full transition-[background] duration-300 ease-in-out light:bg-rich-cerulean-500 bg-rich-cerulean-850 h-[30px] grid grid-cols-[1fr_auto_1fr] items-center select-none pb-[30px]" data-tauri-drag-region>
     <div data-tauri-drag-region></div>
     <span class="text-[16px] leading-7" data-tauri-drag-region>{title ? `UKMCL | ${title}` : win.label != "main" ? `UKMCL | ${win.title()}` : 'UKMCL'}</span>
     <div class="flex justify-end" data-tauri-drag-region>
          {#each ITEMS as i}
               <button onclick={i.click} class="{i.classes} {i.extra}" aria-label={i.label}>
                    {#if !i.wi}
                         <i class="bi {theme.light ? 'bi-sun' : 'bi-moon-stars'}"></i>
                    {:else}
                         <i class="bi {i.i} text-[{i.wi}px]"></i>
                         <!-- <img in:scale={{duration: 300, start: 0.6 }} src={i.src} alt={i.id} width={i.wi} loading="lazy" decoding="async"> -->
                    {/if}
               </button>
          {/each}
     </div>
</div>