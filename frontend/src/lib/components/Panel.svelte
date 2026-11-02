<script lang="ts">
     import { invoke } from "@tauri-apps/api/core";
     import AccountMenu from "./AccountMenu.svelte";

     let baseClasses = "flex justify-center shrink-0 cursor-pointer light:hover:bg-alice-blue-200/50 hover:bg-alice-blue-900/80";
     const ITEMS = [
          { id: "Home", i: 'bi-house-door', endpoint: "/", classes: baseClasses},
          { id: "Instances", i: 'bi-list-ul', endpoint: `/instances?title=${encodeURIComponent("Instances")}`, classes: baseClasses}
     ];
     let settingsUrl = `/settings?title=${encodeURIComponent("Settings")}`;

     async function spawn() {
          await invoke('spawn_store_window', { url: `/store?title=${encodeURIComponent("Store")}` });
     }
</script>

<div class="min-h-full h-auto flex fixed flex-col overflow-y-auto transition-[background] duration-300 ease-in-out light:bg-rich-cerulean-500 bg-rich-cerulean-850 py-1.25 w-[50px]">
     <div class="flex flex-col items-center">
          <a class="cursor-pointer shrink-0 hover:shadow-[0_0_5px_var(--color-sand-dune-200)] rounded-sm transition duration-300 ease-in-out" href="https://github.com/TheUnknownGroup" target="_blank" rel="noopener">
               <img src="/assets/images/icon.png" class="rounded-sm" alt="Icon" width="40" loading="lazy" decoding="async">
          </a>
     </div>
     <div class="flex flex-col items-stretch pt-1.5 pb-0">
          {#each ITEMS as i}
               <a class={i.classes} href={i.endpoint} aria-label={i.id}>
                    <i class="bi {i.i} text-[26px] pt-0.75 pb-0 text-white"></i>
               </a>
          {/each}
          <button class={baseClasses} onclick={spawn} aria-label="Store page">
               <i class="bi bi-puzzle text-[25px] pt-1 pb-0 text-white"></i>
          </button>
     </div>
     <AccountMenu />
     <a href={settingsUrl} class="text-white flex justify-center pt-2 light:hover:bg-alice-blue-200/50 hover:bg-alice-blue-900/80 cursor-pointer bottom-0 fixed w-[50px]" aria-label="Settings button">
          <i class="bi bi-gear-wide-connected text-[25px]"></i>
     </a>

     <div class="pointer-events-none fixed left-[50px] top-[30px] z-10 light:corner-cutout dark-corner-cutout"></div>
</div>