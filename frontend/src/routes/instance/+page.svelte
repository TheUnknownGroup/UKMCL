<script>
     import { onMount } from 'svelte';
     import { getInst, deletes, launch, launchSave, launchServ, edit, instInfo, mods, showSaves, showServs } from '$lib/js/insts';

     let name = $state('Unknown');
     let loader = $state('');
     let version = $state('');
     let loaderVer = $state('');
     let list = $state([]);
     let list2 = $state([]);

     onMount(async () => {
          name = getInst();
          list = await showSaves(name);
          list2 = await showServs(name);
          document.title = name;

          let info = await instInfo(name);
          let main = info.main;
          let loaders = info.loader;
          version = main.minecraft_version;
          loader = loaders.loader;
          loaderVer = loaders.version
     });

     const onDelete = () =>  deletes(name);
     const onLaunch = () => launch(name);

     async function onLaunchSave(world) {
          await launchSave(name, world);
     }
     async function onLaunchServer(server) {
          await launchServ(name, server);
     } 
     
     const onEdit = () => edit(name);
     const onMods = () => mods();
</script>
<style>
     .color {
          color: white
     }
</style>

<div class="border border-white absolute z-50 bg-[rgb(41,41,41)] rounded-lg select-none m-1.25 w-123.5 h-[467px]">
     <h1 class="text-center text-[35px] text-white font-semibold text-ellipsis overflow-hidden whitespace-nowrap pt-2">{name}</h1>
     <div class="border-t-white border border-r-0 border-b-0 border-l-0 w-[60%] mt-[10%] h-[73.9%] float-start">
          <div id="card" class="gap-2 grid pt-2 pr-2 pl-2">
               <p class="color">Launch</p>
               <button onclick={onLaunch} class="items-center justify-center cursor-pointer flex-1 flex p-[8px_15px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] text-white transition-colors duration-300 ease-in-out bg-[#008000] hover:bg-[#00a600]"><img src="/assets/images/play.svg" alt="Launch"></button>
               <p class="color">Delete</p>
               <button onclick={onDelete} class="items-center justify-center flex cursor-pointer flex-1 p-[8px_15px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] text-white transition-colors duration-300 ease-in-out bg-[#a80000] hover:bg-[#ff0000]"><img src="/assets/images/trash.svg" alt="Delete"></button>
               <p class="color">Edit</p>
               <button onclick={onEdit} class="items-center justify-center flex cursor-pointer flex-1 p-[8px_15px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] text-white transition-colors duration-300 ease-in-out bg-[#FFA500] hover:bg-[#FFBB38]" ><img src="/assets/images/tools.svg" alt="Edit"></button>
               <p class="color">Mods</p>
               <button onclick={onMods} class="items-center justify-center flex cursor-pointer flex-1 p-[8px_15px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] text-white transition-colors duration-300 ease-in-out bg-[#297999] hover:bg-[#3AAFDE]" ><img src="/assets/images/mods.svg" alt="Mods"></button>
          </div>
     </div>
     <div class="border-t-white border border-r-0 border-b-0 border-l-white w-[40%] mt-[10%] h-[73.9%] float-right">
          <div id="card" class="gap-2 grid pt-2 pr-2 pl-2">
               <h3 class="color text-center">Instance Info</h3>
               <p class="color">Version: {version}</p>
               {#if loader !== ""}
                    <p class="color">Loader: {loader}</p>
               {/if}
               {#if loaderVer !== ""}
                    <p class="color">Loader Version: {loaderVer}</p>
               {/if}
          </div>
     </div>
</div>

<div class="border border-white absolute z-50 bg-[rgb(41,41,41)] rounded-lg select-none m-1.25 h-[467px] w-136.5 right-0">
     <h1 class="text-center text-[35px] border-white text-white font-semibold text-ellipsis overflow-hidden whitespace-nowrap pt-2 border-r-white border mx-2 border-b-0 rounded-t-lg mt-2 bg-[#02749E]">Worlds</h1>
     <div class="border border-white text-white mx-2 min-h-[34.25%] max-h-[34.25%] overflow-scroll">
          {#each list as name}
               <button onclick={() => { onLaunchSave(name.folder) }} class="cursor-pointer hover:bg-black/30 w-full block py-1.25"><p class="float-left pl-2">{name.folder}</p></button>

          {/each}
     </div>
     <h1 class="text-center text-[35px] border-white text-white font-semibold text-ellipsis overflow-hidden whitespace-nowrap pt-2 border-y-0 border mx-2 bg-[#02749E]">Servers</h1>
     <div class="border border-white text-white mx-2 rounded-b-lg min-h-[34.25%] max-h-[34.25%] overflow-scroll">
          {#each list2 as name} 
               <button onclick={() => { onLaunchServer(name.address) }} class="cursor-pointer hover:bg-black/30 w-full block py-1.25"><p class="float-left pl-2">{name.name}</p><p class=" float-right pr-2 h-full">{name.address}</p></button>
          {/each}
     </div>
</div>
<!-- onclick={location.href='/mods'} -->