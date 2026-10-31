<script lang="ts">
     import { invoke } from "@tauri-apps/api/core";
     import AccountMenu from "./AccountMenu.svelte";

     let baseClasses = "flex justify-center shrink-0 py-2 cursor-pointer hover:bg-alice-blue-900/80";
     const ITEMS = [
          { id: "Home", src: "/assets/images/house-door.svg", endpoint: "/", classes: baseClasses},
          { id: "Instances", src: "/assets/images/list-ul.svg", endpoint: `/instances?title=${encodeURIComponent("Instances")}`, classes: baseClasses}
     ];
     let settingsUrl = `/settings?title=${encodeURIComponent("Settings")}`;

     async function spawn() {
          await invoke('spawn_store_window', { url: `/store?title=${encodeURIComponent("Store")}` });
     }
</script>

<div class="min-h-full h-auto flex fixed flex-col overflow-y-auto bg-rich-cerulean-850 py-1.25 w-[50px]">
     <div class="flex flex-col items-center">
          <a class="cursor-pointer shrink-0 hover:shadow-[0_0_5px_var(--color-sand-dune-200)] rounded-sm transition duration-300 ease-in-out" href="https://github.com/TheUnknownGroup" target="_blank" rel="noopener">
               <img src="/assets/images/icon.png" class="rounded-sm" alt="Icon" width="40" loading="lazy" decoding="async">
          </a>
     </div>
     <div class="flex flex-col items-stretch pt-1.5">
          {#each ITEMS as i}
               <a class={i.classes} href={i.endpoint}>
                    <img src={i.src} alt={i.id} width="25" loading="lazy" decoding="async">
               </a>
          {/each}
          <button class={baseClasses} onclick={spawn}>
               <img src="/assets/images/mods.svg" alt="Mods" width="25" loading="lazy" decoding="async">
          </button>
     </div>
     <AccountMenu />
     <a href={settingsUrl} class="flex justify-center py-2.5 hover:bg-alice-blue-900/80 cursor-pointer bottom-0 fixed w-[50px]">
          <img src="/assets/images/gear-wide-connected.svg" alt="Icon" width="25" loading="lazy" decoding="async">
     </a>

     <div class="pointer-events-none fixed left-[50px] top-[30px] z-10 corner-cutout"></div>
</div>