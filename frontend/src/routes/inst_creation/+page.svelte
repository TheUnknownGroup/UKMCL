<script>
     import { load, create, close } from '$lib/js/inst_c.js';

     let vers = $state([]);
     let vers2 = $state([]);
     let selected = $state('');
     let selected2 = $state('');
     let instName = $state('');
     let loader = $state('vanilla');
     let last = $state(null);
     let loading = $state(false);
     let form;

     const cache = new Map();

     $effect(() => {
          if (loader == last) return;
          last = loader;
          fetches(loader);
     })

     async function fetches(l) {
          if (cache.has(l)){
               const c = cache.get(l);
               vers = c.versions;
               vers2 = c.loaders;
               selected = vers[0] ?? '';
               selected2 = vers2[0] ?? '';
               return;
          }
          loading = true;
          try {
               const { versions, loaders: ls } = await load(l);
               cache.set(l, { versions, loaders: ls });
               vers = versions;
               vers2 = ls;
               selected = vers[0] ?? '';
               selected2 = vers2[0] ?? '';
          } catch (e) {
               console.error('Failed to load version list:', e);
               vers = []; vers2 = [];
               selected = ''; selected2 = '';
          } finally {
               loading = false;
          }
     }

     async function onSubmit(e) {
          e.preventDefault();
          const name = instName.trim();
          if (!name || loading) return;
          try {
               await create(name, selected, loader, selected2);
               form?.reset();
               instName = '';
               loader = 'vanilla';
          } catch (e) {
               console.error(e);
          }
          close(300);
     }
</script>

<style>
     :root {
          background: none;
     }
     
     :root:before {
          opacity: 0;
     }
</style>

<div class="fixed inset-0 z-50 items-center bg-black/70 justify-center flex select-none">
     <div class="bg-[rgb(41,41,41)] border border-white rounded-[10px] p-8.75 h-50 hover:shadow-[0_0_15px_white] transition duration-300 ease-in-out w-120">
          <form bind:this={form} onsubmit={onSubmit}>
               <h3 class="text-[30px] text-[#D9D9D9] pb-4">Instance Name</h3>
               <input type="text" class="bg-[#d9d9d9] text-black rounded-[5px] p-1.75 mb-1.75 mr-1.75 text-[16px]" placeholder="Instance" required bind:value={instName}>
               <select name="ver-list" id="ver-list" class="p-1.5 text-[16px] w-25 text-ellipsis border-none" required bind:value={selected}>
                    {#each vers as id (id)}
                         <option value={id}>{id}</option>
                    {/each}
               </select>
               <br>
               {#if loader !== "vanilla"}
               <select name="ver-list" id="ver-list" class="absolute translate-x-[172%] p-1.5 text-[16px] w-25 text-ellipsis border-none" required bind:value={selected2}>
                    {#each vers2 as id (id)}
                         <option value={id}>{id}</option>
                    {/each}
               </select>
               {/if}
               <div class="fixed translate-x-[265%] translate-y-[-72%] text-white">
                    <input type="radio" id="vanilla" value="vanilla" bind:group={loader} required>
                    <label for="vanilla">Vanilla</label>
                    <div class="float-right"><img src="/assets/images/Grass_Block.svg" alt="Vanilla" width="24" ></div><br>
                    <input type="radio" id="fabric" value="fabric" bind:group={loader} required>
                    <label for="fabric">Fabric</label>
                    <div class="float-right"><img src="/assets/images/fabricmc.svg" alt="Fabric" width="24"></div><br>
                    <input type="radio" id="quilt" value="quilt" bind:group={loader} required>
                    <label for="quilt">Quilt</label>
                    <div class="float-right"><img src="/assets/images/quiltmc.svg" alt="Quilt" width="24"></div><br>
                    <input type="radio" id="forge" value="forge" bind:group={loader} required>
                    <label for="forge">Forge</label>
                    <div class="float-right"><img src="/assets/images/forgemc.svg" alt="Forge" width="24"></div><br>
                    <input type="radio" id="neoforge" value="neoforge" bind:group={loader} required>
                    <label for="neoforge">NeoForge</label>
                    <div class="float-right pl-[15px]"><img src="/assets/images/neoforged.svg" alt="Neoforged" width="24"></div>
               </div>
               <button type="button" onclick={() => close(800)} class="rounded-[5px] p-[5px_6px_4px_6px] text-[14px] text-white shadow-[0_0_10px_rgba(0,0,0,0.7)] bg-[#a80000] mr-1.75 hover:bg-red-600 cursor-pointer">Cancel</button>
               <button type="submit" class="rounded-[5px] p-[5px_6px_4px_6px] text-[14px] text-white shadow-[0_0_10px_rgba(0,0,0,0.7)] bg-[#00a600] hover:bg-[#005c00] cursor-pointer">Create</button>
          </form>
     </div> 
</div>