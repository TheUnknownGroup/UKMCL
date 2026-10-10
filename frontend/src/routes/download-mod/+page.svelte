<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { getCurrentWindow } from '@tauri-apps/api/window';

    function getInfo() {
         const params = new URLSearchParams(window.location.search);
         return params.get('name') || 'Unknown';
    }
    
    function getVer() {
         const params = new URLSearchParams(window.location.search);
         return params.get('ver') || 'Unknown';
    }
    
    function getInst() {
         const params = new URLSearchParams(window.location.search);
         return params.get('inst_name') || 'Unknown';
    }

    function getLoad() {
         const params = new URLSearchParams(window.location.search);
         return params.get('loader') || 'Unknown';
    }

    let info = $state('');
    let version = $state('');
    let loader = $state('');
    let inst = $state('');
    let infos = $state([]);
    let selected = $state('');
    
    onMount(async () => {
         info = getInfo();
         version = getVer();
         inst = getInst();
         loader = getLoad();
         try {
              infos = await invoke('list_mod_info', { projectId: info, gameVer: version });
         } catch (e) {
              console.error(e);
         }
    });

    async function download(url, file) {
         try {
              await invoke('downloads', { url, file, instName: inst});
              await invoke('delete_mod_json', { id: info, ver: version });
              setTimeout(() => getCurrentWindow().close(), 800);
         } catch (e) {
              console.error(e);
         }
    }

    async function close() {
         try {
              await invoke('delete_mod_json', { id: info, ver: version });
              setTimeout(() => getCurrentWindow().close(), 800); 
         } catch (e) {
              console.error(e);
         }
    }

    async function downDeps(selecteds) {
          try {
               const file = selecteds.files.find(f => f.primary) ?? selecteds.files[0];
               await invoke('downloads', { url: file.url, file: file.filename, instName: inst });
               for (const dep of selecteds.dependencies ?? []) {
                    if(dep.dependency_type !== "required") continue;
                    if (!dep.project_id) continue;
                    await invoke('download_depss', { versionId: dep.version_id, projectId: dep.project_id, game: version, loader, instName: inst })
               }
               await invoke('delete_mod_json', { id: info, ver: version });
               setTimeout(() => getCurrentWindow().close(), 800);
          } catch (e) {
               console.error(e);
          }
    }

    let deps = $derived(
         selected?.dependencies?.filter(d => d.dependency_type == "required") ?? []
    );
</script>

<style>
     :root {
          background: none;
     }

     :root:before {
          opacity: 0;
     }
</style>

<div class="fixed inset-0 z-50 items-center justify-center flex bg-black/70">
     <div class="border border-white rounded-lg m-2 p-2 bg-[rgb(41,41,41)] w-122 flex align-middle h-20">
          <select class="w-118 h-8" bind:value={selected}>
               {#each infos as i (i.id)}
                    <option class="text-ellipsis" value={i}>{i.name} ({i.version_number})</option>
               {/each}
          </select>
          <div class="absolute right-26 bottom-40">
               <button onclick={() => close()} class="mr-0.25 px-2 text-white hover:bg-[#F26363] rounded-lg bg-[#AB1B1B] cursor-pointer pt-0.25">Cancel</button>
               {#if deps.length > 0}
                    <button onclick={() => downDeps(selected)} class="text-white cursor-pointe px-2 rounded-lg bg-[#238510] hover:bg-[#3B9E2C] pt-0.25 cursor-pointer"> {!selected ? "Choose a version." : "Download With Dependencies"} </button>
               {:else}
                    <button onclick={() => { const file = selected.files.find(f => f.primary) ?? selected.files[0]; download(file.url, file.filename);}} class="text-white cursor-pointe px-2 rounded-lg bg-[#238510] hover:bg-[#3B9E2C] pt-0.25 cursor-pointer">{!selected ? "Choose a version." : "Download"}</button>
               {/if}
          </div>
     </div>
</div>