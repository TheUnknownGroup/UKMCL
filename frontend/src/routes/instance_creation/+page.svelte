<script lang="ts">
    import { load, create, close, loaderList, def } from "$lib/ts/instance";
    let versions: any = $state([]);
    let loaderVersions = $state([]);
    let selectedVersion = $state('');
    let selectedLoaderVersion = $state('');
    let name = $state('');
    let currentLoader = $state('vanilla');
    let lastLoader = $state('');
    let loading = $state(false);
    const cache = new Map();
    const loaderCache = new Map();

    $effect(() => {
         if (!currentLoader || currentLoader === lastLoader) return;
         lastLoader = currentLoader;
         fetching(currentLoader);
    });

    $effect(() => { if (currentLoader === "neoforge") loadLoaders(currentLoader, selectedVersion); });

    async function fetching(l: any) {
         loaderVersions = []; selectedVersion = ''; selectedLoaderVersion = '';
         let c = cache.get(l);
         if (!c) {
              loading = true;
              try {
                   c = await load(l);
                   cache.set(l, c);
              } catch (e) {
                   console.error("Failed to load version list: ", e);
                   c = { versions: [], loaders: [] };
              } finally {
                   if (l === currentLoader) loading = false;
              }
         }

         if (l !== currentLoader) return;
         versions = c.versions;
         selectedVersion = def(versions, l);
         if (l !== "neoforge") {
              loaderVersions = c.loaders;
              selectedLoaderVersion = c.loaders[0] ?? '';
         }
    }

    async function loadLoaders(l: string, mc: string) {
         if (!mc) { loaderVersions = []; selectedLoaderVersion = ''; return; }
         const key = `${l}:${mc}`;
         let list = loaderCache.get(key);
         if (!list) {
              try {
                   list = await loaderList(l, mc);
                   loaderCache.set(key, list);
              } catch (e) {
                   console.error("Failed to load loader versions", e);
                   list = [];
              }
         }
         if (l !== currentLoader || mc !== selectedVersion) return;
         loaderVersions = list;
         selectedLoaderVersion = list[0] ?? '';
    }

    let creating = $state(false);

    async function onSubmit(e: any) {
         e.preventDefault();
         const final = name.trim();
         if (!final || loading) return;
         creating = true;
         try {
              await create(final, selectedVersion, currentLoader, selectedLoaderVersion);
              name = "";
              currentLoader = 'vanilla';
              close(300);
         } catch (e) {
              console.error(e);
         } finally {
              creating = false;
         }
    }
</script>

<div class="absolute items-center justify-center flex select-none top-[30%] left-[16%]">
     <div class="bg-creation border border-white rounded-lg p-8.75 h-50 w-140">
          <form onsubmit={onSubmit}>
               <h3 class="text-[30px] text-creation-2 pb-4">Instance Name</h3>
               <input type="text" class="bg-creation-2 text-black rounded-md p-1.75 mb-1.75 mr-1.75 text-[16px]" placeholder="Instance" required bind:value={name}>
               {#if currentLoader !== "forge" && currentLoader !== "neoforge"}
                    <select name="ver-list" id="ver-list" class="p-1.5 text-[16px] w-25 text-ellipsis border-none text-black" required bind:value={selectedVersion}>
                         {#each versions as i}
                              <option value={i.id}>{i.id}</option>
                         {/each}
                    </select>
               {:else if currentLoader === "forge" || currentLoader === "neoforge"}
                    <select name="ver-list" id="ver-list" class="p-1.5 text-[16px] w-25 text-ellipsis border-none text-black" required bind:value={selectedVersion}>
                         {#each versions.filter((v: any) => v.ver_type === "release")  as i}
                              <option value={i.id}>{i.id}</option>
                         {/each}
                    </select>
               {/if}
               <br>
               {#if currentLoader !== "vanilla" && currentLoader !== "forge"}
                    <select name="loader-list" id="loader-list" class="absolute translate-x-[209%] p-1.5 text-[16px] w-25 text-ellipsis border-none text-black" required bind:value={selectedLoaderVersion}>
                         {#each loaderVersions as i}
                              <option value={i}>{i}</option>
                         {/each}
                    </select>
               {/if}

               <div class="fixed translate-x-[265%] translate-y-[-76%] text-white text-[20px]">
                    <input type="radio" id="vanilla" value="vanilla" bind:group={currentLoader} required>
                    <label for="vanilla">Vanilla</label>
                    <div class="float-right"><img src="/assets/images/Grass_Block.svg" alt="Vanilla" width="24" loading="lazy" decoding="async"></div><br>
                    <input type="radio" id="fabric" value="fabric" bind:group={currentLoader} required>
                    <label for="fabric">Fabric</label>
                    <div class="float-right"><img src="/assets/images/fabricmc.svg" alt="FabricMC" width="24" loading="lazy" decoding="async"></div><br>
                    <input type="radio" id="quilt" value="quilt" bind:group={currentLoader} required>
                    <label for="quilt">Quilt</label>
                    <div class="float-right"><img src="/assets/images/quiltmc.svg" alt="QuiltMC" width="24" loading="lazy" decoding="async"></div><br>
                    <input type="radio" id="forge" value="forge" bind:group={currentLoader} required>
                    <label for="forge">Forge</label>
                    <div class="float-right"><img src="/assets/images/forgemc.svg" alt="ForgeMC" width="24" loading="lazy" decoding="async"></div><br>
                    <input type="radio" id="neoforge" value="neoforge" bind:group={currentLoader} required>
                    <label for="neoforge">Neoforge</label>
                    <div class="float-right pl-3"><img src="/assets/images/neoforged.svg" alt="Neoforged" width="24" loading="lazy" decoding="async"></div>
               </div>
               <button type="button" onclick={() => close(200)} class="rounded-[5px] px-[6px] pt-[5px] pb-[4px] text-[14px] shadow-[0_0_10px_rgba(0,0,0,0.7)] bg-scarlet-rush-600 hover:bg-scarlet-rush-400 cursor-pointer">Cancel</button>
               <button type="submit" class="rounded-[5px] px-[6px] pt-[5px] pb-[4px] text-[14px] shadow-[0_0_10px_rgba(0,0,0,0.7)] bg-fern-600 hover:bg-fern-400 cursor-pointer" disabled={creating}>Create</button>
          </form>
     </div>
</div>