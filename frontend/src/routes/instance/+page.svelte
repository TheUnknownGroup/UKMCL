<script lang="ts">
    import { getInstance, instanceInfo, deletes, launch, launchSave, launchServer, open, showSaves, showServers } from "#lib/ts/instance.ts";
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    let name = $state('');
    let version = $state('');
    let loader = $state('');
    let loaderVer = $state('');
    let mods = $state('');
    let savesList: any = $state([]);
    let serverList: any = $state([]);
    
    onMount(async () => {
         try {
              name = getInstance();
              savesList = await showSaves(name);
              serverList = await showServers(name);
              const info: any = await instanceInfo(name);
              version = info.main.minecraft_version;
              loader = info.loader.loader;
              loaderVer = info.loader.version;
         } catch (e) {
              console.error(e);
         }
    });
    const items = $derived([
         { id: "Name: ", label:  name},
         { id: "Version: ", label: version},
         { id: "Loader: ", label: loader || 'vanilla'},
         { id: "Loader Version: ", label: loaderVer || 'No version because this instance is vanilla'},
    ]);

    async function spawn() {
         await invoke('spawn_store_window', { url: `/store?title=${encodeURIComponent("Store")}` });
    }
    const onDelete = async () => {
         try {
              await deletes(name);
              setTimeout(() => getCurrentWindow().close(), 300); 
         } catch (e) {
              console.error(e);
         }
    };
</script>

<div class="mt-7.5 text-left w-194 absolute select-none bg-alice-blue-600/70 h-[45%]">
     <h1 class="text-[35px] text-white font-semibold px-2 py-0 text-ellipsis overflow-hidden whitespace-nowrap pt-2">Worlds</h1>
     <div class="border border-alice-blue-700/70 text-white min-h-[76.50%] max-h-[76.50%] overflow-scroll rounded-md mx-1 bg-alice-blue-600/70">
          {#each savesList as n}
               <button onclick={async () => await launchSave(name, n.folder)} class="cursor-pointer hover:bg-black/30 w-full block py-1.25"><p>{n.folder}</p></button>
          {/each}
     </div>

</div>

<div class="bg-alice-blue-600/90 absolute mt-[30.826691%] text-left w-194 h-[52%]">
     <h1 class="text-[35px] text-white font-semibold px-2 py-0 text-ellipsis overflow-hidden whitespace-nowrap pt-2 ">Servers</h1>
     <div class="border border-alice-blue-700/70 text-white mx-1 min-h-[76%] max-h-[76%] overflow-scroll rounded-md bg-alice-blue-700/50">
          {#each savesList as n}
               <button onclick={async () => await launchServer(name, n.address)} class="cursor-pointer hover:bg-black/30 w-full block py-1.25"><p>{n.address}</p></button>
          {/each}
     </div>
</div>

<div class="light:bg-ink-black-500 transition-[background] duration-300 ease-in-out bg-alice-blue-700 mt-7.5 h-full w-57 ml-auto text-white">
     <div class="select-none space-y-3">
          <h3 class="text-[20px] text-center border-b transition-[background] duration-300 ease-in-out light:border-ink-black-400/50 border-ink-black-700/50">Instance<br>Information</h3>
          {#each items as i}
               <h3 class="text-[17px] pl-1.5 text-left">{i.id} <span class=" text-white/50">{i.label}</span></h3>
          {/each}
     </div>
     <h3 class="absolute bottom-[45%] pl-1.25 text-[25px]">Instance Controls</h3>
     <div class="text-white">
          <div class="absolute bottom-1.5 gap-2 grid px-2 w-[22.7%]">
               <p class="pb-0 mb-0">Launch</p>
               <button onclick={() => launch(name)} class="cursor-pointer border pt-1 light:bg-fern-400 bg-fern-600" aria-label="launch"><i class="bi bi-play-fill"></i></button>
               <p class="pb-0 mb-0">Delete</p>
               <button onclick={onDelete} class="cursor-pointer border pt-1 light:bg-fern-400 bg-fern-600" aria-label="launch"><i class="bi bi-trash"></i></button>
               <p class="pb-0 mb-0">Edit</p>
               <button onclick={() => open(name)} class="cursor-pointer border pt-1 light:bg-fern-400 bg-fern-600" aria-label="launch"><i class="bi bi-tools"></i></button>
               <p class="pb-0 mb-0">Mods</p>
               <button onclick={spawn} class="cursor-pointer border pt-1 light:bg-fern-400 bg-fern-600" aria-label="launch"><i class="bi bi-puzzle"></i></button>
          </div>
     </div>
</div>