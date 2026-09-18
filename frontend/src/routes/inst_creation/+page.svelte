<script>
     import { onMount } from 'svelte';
     import { load, create, close } from '$lib/js/inst_c.js';

     let vers = $state([]);
     let selected = $state('');
     let instName = $state('');
     let form;

     onMount(async () => {
          try {
               vers = await load();
               if (vers.length) selected = vers[0];
          } catch (e) {
               console.error('Failed to load version list:', e);
          }
     })

     async function onSubmit(e) {
          e.preventDefault();
          const name = instName.trim();
          if (!name) return;
          try {
               await create(name, selected);
               form?.reset();
               instName = '';
               selected = vers[0] ?? '';
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
     <div class="bg-[rgb(41,41,41)] border border-white rounded-[10px] p-8.75 h-50 hover:shadow-[0_0_15px_white] transition duration-100 ease-out">
          <form bind:this={form} onsubmit={onSubmit}>
               <h3 class="text-[30px] text-[#D9D9D9] pb-7">Instance Name</h3>
               <input type="text" class="bg-[#d9d9d9] text-black rounded-[5px] p-1.75 mb-1.75 mr-1.75 text-[16px]" placeholder="Instance" required bind:value={instName}>
               <select name="ver-list" id="ver-list" class="p-1.5 text-[16px] w-25 text-ellipsis border-none" required bind:value={selected}>
                    {#each vers as id (id)}
                         <option value={id}>{id}</option>
                    {/each}
               </select>
               <br>
               <button type="button" onclick={() => close(800)} class="rounded-[5px] p-[5px_6px_4px_6px] text-[14px] text-white shadow-[0_0_10px_rgba(0,0,0,0.7)] bg-[#a80000] mr-1.75 hover:bg-red-600 cursor-pointer">Cancel</button>
               <button type="submit" class="rounded-[5px] p-[5px_6px_4px_6px] text-[14px] text-white shadow-[0_0_10px_rgba(0,0,0,0.7)] bg-[#00a600] hover:bg-[#005c00] cursor-pointer">Create</button>
          </form>
     </div> 
</div>