<script>
     import { invoke } from '@tauri-apps/api/core';
     import { onMount } from 'svelte';
     
     let names = $state([]);
     let loading = $state(true);

     let minHeight = $state('99%');
     let maxHeight = $state('88vh');

     async function getLatest() {
          loading = true;
          try {
               names = await invoke('get_command');
          } catch (err) {
               console.error(err);
          } finally {
               loading = false
          }
     }

     let chosen = $state('');
     let query = $state('');

     let func = $state([]);
     let modName = $state([]);

     let index = $state(0);

     let versions = $state('');
     let loader = $state('');

     async function loadMods(loader = '', version = '') {
          modName = await invoke("search", { query: query, loader: loader, game: version, offset: index * 50});
     }

     $effect(() => {
          const q = query;
          const name = chosen;
          index = 0;
     })

     let timer;
     $effect(() => {
          const q = query;
          const name = chosen;
          const p = index;
          clearTimeout(timer);
          
          timer = setTimeout(async () => {
               if (!name) { loadMods('', ''); return; };
               const info = await invoke("inst_info", { name: name });
               loader = info.loader.loader;
               versions = info.main.minecraft_version;
               loadMods(info.loader.loader, info.main.minecraft_version);
          }, 300);
     });
     
     onMount(() => {
          getLatest();
          loadMods('', '');
     });

     function next() {
          index += 1;
     }
     
     function prev() {
          index -= 1;
     }

     function formats(n) {
          if (n >= 1_000_000) return (n / 1_000_000).toFixed(1) + 'M';
          if (n >= 1_000) return (n / 1_000).toFixed(1) + 'k';
          return String(n)
     }

     async function download(id, namees) {
          console.log(`download_called: ${id}, and ${versions}`);
          if (!chosen) {console.log("no instance found"); return;}
          const info = await invoke("inst_info", { name: chosen });
          let url = `/download-mod?name=${encodeURIComponent(id)}&ver=${encodeURIComponent(versions)}&inst_name=${encodeURIComponent(chosen)}&loader=${encodeURIComponent(loader)}`;
          let ver = versions.replaceAll('.', '_');
          let label = `mod-${namees}_${ver}`;
          try {
               await invoke("save_mod", { projectId: id, game: versions, loader: loader });
               await invoke("spawn_mods", { label, url, name: namees });
          } catch (e) {
               console.error(e);
          }
     }
</script>

<div class="flex-1 overflow-auto">
     <div class="fixed right-0 border m-1.25 p-2 rounded-lg border-white bg-black/30 flex z-1" style:min-width={minHeight}>
          <input class="text-white border-white border rounded-md pl-1.25 w-[550px]" type="text" placeholder="Search" bind:value={query}>
          <div class="text-white pl-[55px]">
               <button onclick={prev} class="cursor-pointer">Prev</button>
               <span>Page {index + 1}</span>
               <button onclick={next} class="cursor-pointer">Next</button>
          </div>
          {#if loading}
               <p class="pr-1.5 w-40">No instances yet.</p>
          {:else if names.length === 0}
               <p class="pr-1.5 w-40">No instances yet.</p>
          {:else}
               <select bind:value={chosen} class="w-35 ml-auto">
                    {#each names as name}
                         <option value={name}>{name}</option>
                    {/each}
               </select>
          {/if}
     </div>
     
     <div class="m-1.25 p-2 border border-white rounded-lg mt-[65px] overflow-scroll bg-[rgb(41,41,41)] select-none" style:max-height={maxHeight} style:min-height={maxHeight}>
          {#each modName as names}
               <div class="border border-gray-400 not-last:mb-[10px] overflow-hidden max-h-[80px] rounded-[5px]">
                    <span class="flex p-1.25"><img src={names.icon_url} alt="Icon" width="32"/><p class="text-white pl-[5px]">{names.title}</p></span>
                    <span class="text-white pl-1.25 flex"><img src="/assets/images/download.svg" alt="Downloads"><p class="pl-[5px]">{formats(names.downloads)}</p></span>
                    <button onclick={() => { download(names.project_id, names.slug); }} class="flex cursor-pointer -translate-y-full translate-x-[1090%] rounded-lg"><p class="text-white">Download</p></button>
               </div>
          {/each}
     </div>
</div>