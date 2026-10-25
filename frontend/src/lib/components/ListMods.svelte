<script>
     import { invoke } from '@tauri-apps/api/core';
     import { onMount } from 'svelte';
     import DownloadedToast from './DownloadedToast.svelte';

     function getInfo() {
          const params = new URLSearchParams(window.location.search);
          return params.get('name') || '';
     }
     
     let names = $state([]);
     let loading = $state(true);
     let downloads = $state(false);

     let currentDownloader = $state('modrinth');
     let maxHeight = $state('89.3vh');
     let maxHeight2 = $state('89.3vh');

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
     let selected = $state('');
     let vers = $state.raw([]);
     let modName = $state.raw([]);

     let index = $state(0);
     let hits = $state(0);

     let versions = $state('');
     let mod = $state('');
     let shader = $state('');
     let pack = $state('fabric');
     let tab = $state('mods');
     const isMods = $derived(tab === 'mods');
     const isPacks = $derived(tab === 'modpacks');
     const isShade = $derived(tab === 'shaders');

     let name = $derived(chosen);

     const SHADER_LOADERS = [
          { id: 'iris', label: 'Iris Shaders', img: 'https://cdn.modrinth.com/data/YL57xq9U/18d0e7f076d3d6ed5bedd472b853909aac5da202_96.webp' },
          { id: 'optifine', label: 'Optifine', img: 'https://ia600603.us.archive.org/8/items/opti-fine-1.7.10-hd-u-e-7_202409/optifine-logo.jpg' },
          { id: 'vanilla-like', label: 'Vanilla', img: '/assets/images/Grass_Block.svg' },
          { id: 'canvas', label: 'Canvas', img: null },
     ];

     const MODPACK_LOADER = [
          { id: 'fabric', label: 'Fabric', img: '/assets/images/fabricmc.svg' },
          { id: 'quilt', label: 'Quilt', img: '/assets/images/quiltmc.svg' },
          { id: 'neoforge', label: 'NeoForged', img: '/assets/images/neoforged.svg' }
     ]
     let req = 0;
     async function load() {
          const id = ++req;
          const kind = isMods ? 'mods' : isPacks ? 'modpacks' : 'shaders'; 
          const loader = isShade ? shader : isPacks ? pack : mod;
          const game = isPacks ? selected : versions;
          const offset = index * 50;
          try {
               const r = currentDownloader === 'curseforge' ? await invoke('curse', { kind, query, loader, game, offset }) : await invoke(isMods ? 'modri' : isPacks ? 'modri_packs' : 'modri_shades', { query, loader, game, offset });
               if (id !== req) return;
               modName = r.hits;
               hits = r.total_hits;
          } catch (e) {
               if (id !== req) return;
               modName = []; hits = 0;
               console.error(e);
          }
     }

     $effect(() => {
          query;
          chosen;
          pack;
          selected;
          currentDownloader;
          index = 0;
     });

     const def = (list, l) => {
          if (l === "forge" || l === "neoforge") {
               return list.find(v => v.ver_type === "release")?.id ?? '';
          }
          return list[0]?.id ?? '';
     };
     let last = '';
     let timer;

     $effect(() => {
          const p = pack;
          if (!p) return;
          (async () => {
               let info;
               try {
                    info = await invoke('get', { loader: p });
               } catch {
                    await invoke('load_versions', { loader: p });
                    info = await invoke('get', { loader: p });
               }
               if (p !== pack) return;
               vers = info;
               if (p !== last) { selected = def(info, p); last = p; }
          })().catch(console.error);
     });
     
     $effect(() => {
          query; chosen; tab; mod; index; shader; pack; selected; currentDownloader;
          clearTimeout(timer);
          timer = setTimeout(async () => {
               if (!chosen && !isPacks) { await load(); return; };
               if (!pack && isPacks) return;
               if (chosen && !isPacks) {
                    const info = await invoke("inst_info", { name: chosen });
                    versions = info.main.minecraft_version;
                    if (isMods) {
                         mod = info.loader.loader;
                    }
               }
               await load();
          }, 300);
          return () => clearTimeout(timer);
     });
     
     onMount(async () => {
          getLatest();
          chosen = getInfo();
     });

     const pages = $derived(Math.max(1, Math.ceil(hits / 50)));
     const next = () => { if (index + 1 < pages) index += 1; };
     const prev = () => { index = Math.max(0, index - 1) };

     function formats(n) {
          if (n >= 1_000_000) return (n / 1_000_000).toFixed(2) + 'M';
          if (n >= 1_000) return (n / 1_000).toFixed(2) + 'k';
          return String(n)
     }

     function setShade(id) {
          shader = shader === id ? '' : id;
     }
     function setPack(id) {
          pack = id;
     }
     
     function setTab(next) {
          if (tab === next) return;
          tab = next;
          if (next !== 'shaders') shader = '';
          if (next !== 'mods') mod = '';
          if (next === 'modpacks' && !pack) pack = 'fabric';
     }

     async function download(kind, id, slug) {
          if (!chosen) { console.warn('no instance chosen'); return; }
          let loader = kind === "shaders" ? shader : mod;
          let url = `/download-${kind}?name=${encodeURIComponent(id)}&ver=${encodeURIComponent(versions)}&inst_name=${encodeURIComponent(chosen)}&loader=${encodeURIComponent(loader)}`;
          let ver = versions.replaceAll('.', '_');
          let label = `${kind}-${slug}_${ver}`;
          try {
               await invoke("save_mod", { projectId: id, game: versions, loader: loader });
               await invoke("spawn_mods", { label, url, name: slug });
               downloads = true;
          } catch (e) {
               console.error(e);
          }
          downloads = false;
     }

     async function downloadPacks(id, slug, title) {
          if (!pack) { return; }
          if (!selected) { return; }
          let url = `/download-modpacks?name=${encodeURIComponent(id)}&ver=${encodeURIComponent(selected)}&loader=${encodeURIComponent(pack)}`;
          let ver = selected.replaceAll('.', '_');
          let label = `modpacks-${slug}_${ver}`;
          try {
               await invoke("save_mod", { projectId: id, game: selected, loader: pack });
               await invoke("spawn_mods", { label, url, name: slug });
               downloads = true;
          } catch (e) {
               console.error(e);
               downloads = false;
          }
          downloads = false;
     }

     let busy = $state('');
     let skipped = $state([]);

     async function installCurse(id) {
          const kind = isMods ? 'mods' : isPacks ? 'modpacks' : 'shaders';
          if (!isPacks && !chosen) return;
          busy = id;
          skipped = [];
          try {
               skipped = await invoke('cf_install', { kind, projectId: id, game: isPacks ? selected : versions, loader: isShade ? '' : isPacks ? pack : mod, instName: chosen});
               downloads = true;
          } catch (e) {
               console.error(e);
          } finally {
               busy = '';
               downloads = false;
          }
     }
     
     const downM = (id, slug) => currentDownloader === 'curseforge' ? installCurse(id) : download('mod', id, slug);
     const downP = (id, slug, title) => currentDownloader === 'curseforge' ? installCurse(id) : downloadPacks(id, slug, title);
     const downS = (id, slug) => currentDownloader === 'curseforge' ? installCurse(id) : download('shaders', id, slug);

     const shadow = (on) => on ? 'shadow-[0_0_10px_#C7C7C7]' : '';
     const borderColor = 'border-[#7A7A7A]';
</script>

<div class="flex-1 overflow-auto">
     <div class="fixed border m-1.25 p-2 rounded-lg {borderColor} bg-black/30 flex z-1">
          <input class="text-white {borderColor} border rounded-md pl-1.25 w-[780px] mr-5" type="text" placeholder="Search" bind:value={query}>
          {#if isPacks}
               <div class="m-auto"><p class="pr-1.5 select-none text-white w-82.25">Creating a Modpack?</p></div>
          {:else if loading || names.length === 0}
               <p class="pr-1.5 w-40">No instances yet.</p>
          {:else}
               <select bind:value={chosen} class="w-82.25 text-[20px]">
                    {#each names as name}
                         <option value={name}>{name}</option>
                    {/each}
               </select>
          {/if}
     </div>
     
     <div class="m-1.25 p-2 border {borderColor} rounded-lg mt-[68px] w-[800px] overflow-scroll bg-[rgb(41,41,41)] select-none float-left" style:height={maxHeight}>
          {#each modName as names}
               <div class="border {borderColor} not-last:mb-[10px] overflow-hidden max-h-[80px] rounded-[5px] scroll-smooth">
                    <span class="flex p-1.25"><img src={names.icon_url} alt="Icon" width="32" height="32" loading="lazy" decoding="async" /><p class="text-white pl-[5px]">{names.title}</p></span>
                    <span class="text-white pl-1.25 flex"><img src="/assets/images/download.svg" alt="Downloads"><p class="pl-[5px]">{formats(names.downloads)}</p></span>
                    {#if isMods}
                         <button onclick={() => { downM(names.project_id, names.slug); }} class="flex cursor-pointer -translate-y-full translate-x-[778%] rounded-lg"><p class="text-white">Download</p></button>
                    {:else if isPacks}
                         <button onclick={() => { downP(names.project_id, names.slug, names.title); }} class="flex cursor-pointer -translate-y-full translate-x-[778%] rounded-lg"><p class="text-white">Download</p></button>
                    {:else if isShade}
                         <button onclick={() => { downS(names.project_id, names.slug); }} class="flex cursor-pointer -translate-y-full translate-x-[778%] rounded-lg"><p class="text-white">Download</p></button>
                    {/if}        
               </div>
          {/each}
     </div>

     <div class="m-1.25 border {borderColor} float-right bg-[rgb(41,41,41)] w-[34%] rounded-lg mt-[68px]" style:max-height={maxHeight2} style:min-height={maxHeight2}>
          <h1 class="text-white text-[30px] p-2 pl-5">Controls</h1>
          <div class="text-white {borderColor} px-0 mx-0 grid grid-cols-[repeat(auto-fit,minmax(30px,1fr))]">
               <button onclick={() => currentDownloader = 'modrinth'} class="cursor-pointer border-x-0 border-b-0 border border-t-[#7a7a7a] py-2 flex justify-center hover:bg-black/30 transition duration-300 ease-in-out {shadow(currentDownloader == "modrinth")}"><img src="/assets/images/modrinth.svg" width="32" alt="Modrinth"></button>
               <button onclick={() => currentDownloader = 'curseforge'} class="cursor-pointer border border-t-[#7a7a7a] border-x-0 border-b-0 py-2 flex justify-center hover:bg-black/30 transition duration-300 ease-in-out {shadow(currentDownloader == "curseforge")}"><img src="/assets/images/curseforge.svg" width="32" alt="Curseforge"></button>
          </div>
          <div class="text-white border-y-[#7A7A7A] border border-x-0 px-0 mx-0 grid grid-cols-[repeat(auto-fit,minmax(30px,1fr))]">
               <button onclick={() => setTab('mods')} class="cursor-pointer border-r-[#7A7A7A] border border-y-0 border-l-0 py-2 block hover:bg-black/30 transition duration-300 ease-in-out {shadow(isMods)}">Mods</button>
               <button onclick={() => setTab('modpacks')} class="cursor-pointer border-r-[#7A7A7A] border border-y-0 border-l-0 py-2 block hover:bg-black/30 transition duration-300 ease-in-out {shadow(isPacks)}">Modpacks</button>
               <button onclick={() => setTab('shaders')} class="cursor-pointer py-2 block hover:bg-black/30 transition duration-300 ease-in-out {shadow(isShade)}">Shaders</button>
          </div>

          {#if isPacks}
               <div class="text-white border-y-[#7a7a7a] border border-x-0 px-0 mx-0 grid grid-cols-[repeat(auto-fit,minmax(30px,1fr))] text-[15px] border-t-0">
                    {#each MODPACK_LOADER as m}
                         <button onclick={() => setPack(m.id)} class="cursor-pointer border-r-[#7a7a7a] last:border-r-0 border border-y-0 border-l-0 py-2 flex justify-center hover:bg-black/30 transition duration-300 ease-in-out {shadow(pack === m.id)}"><img src={m.img} alt={m.label} width="32"></button>
                    {/each}
               </div>

               <div class="text-white border-y-[#7a7a7a] border border-x-0 p-2 mx-0 text-[15px] border-t-0">
                    <select name="ver-list" required bind:value={selected} class="text-black w-full">
                         {#if pack === "fabric" || pack === "quilt"}
                              {#each vers as v}
                                   <option value={v.id}>{v.id}</option>
                              {/each}
                         {:else if pack === "neoforge" || pack === "forge"}
                              {#each vers.filter(v => v.ver_type === "release" ) as i (i.id) }
                                   <option value={i.id}>{i.id}</option>
                              {/each}
                         {/if}
                    </select>
               </div>
          {/if}

          {#if isShade } 
               <div class="text-white border-y-[#7a7a7a] border border-x-0 px-0 mx-0 grid grid-cols-[repeat(auto-fit,minmax(30px,1fr))] text-[20px] border-t-0">
                    {#each SHADER_LOADERS as s}
                         <button onclick={() => setShade(s.id)} class="cursor-pointer border-r-[#7a7a7a] last:border-r-0 border border-y-0 border-l-0 py-2 flex justify-center hover:bg-black/30 transition duration-300 ease-in-out {shadow(shader === s.id)}">
                              {#if s.img}
                                   <img src={s.img}  width="32" alt={s.label}>
                              {:else}
                                   {s.label}
                              {/if}
                         </button>
                    {/each}
               </div>
          {/if}

          <div class="text-white">
               <h3 class="text-[30px] p-2 pl-5">Instance Info</h3>
               {#if (isMods || isShade) && chosen}
                    <div class="border {borderColor} border-x-0 p-2">
                         <p>Version: {versions}</p>
                         <p>Loader: {#if (isMods)} {mod} {:else if (isShade)} {shader} {/if}</p>
                    </div>
               {:else if !chosen && (isMods || isShade)}
                    <div class="border {borderColor} border-x-0 p-2 flex justify-center"> No Instance Chosen</div>
               {/if}

               {#if isPacks}
                    <div class="border {borderColor} border-x-0 p-2">
                         <p>Loader: {pack}</p>
                         <p>Version: {selected}</p>
                    </div>
               {/if}
          </div>

          <DownloadedToast bind:downloads bind:name />

          <div class="text-white px-5 ml-auto select-none absolute bottom-4 flex">
               <button onclick={prev} class="cursor-pointer border {borderColor} rounded-lg px-1.75">Prev</button>
               <div class="w-52 justify-center flex">Page {index + 1} of {pages}</div>
               <button onclick={next} class="cursor-pointer border {borderColor} rounded-lg px-1.75">Next</button>
          </div>
     </div>
</div>