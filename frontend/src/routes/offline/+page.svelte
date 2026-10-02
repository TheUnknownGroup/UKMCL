<script>
     import { account, close } from "$lib/js/offline.js";
     let name = $state('');
     let form;

     async function onSubmit(e) {
          e.preventDefault();
          const names = name.trim();
          if (!names) return;
          try {
               await account(names);
               form?.reset();
               name = '';
          } catch (e) {
               console.error(e);
          }
          close(300);
     }
</script>

<style>
     :root {
          background: #5C5C5C;
     }
     :root:before {
          opacity: 0;
     }
</style>

<div class="fixed inset-0 z-50 items-center justify-center flex select-none">
     <div class=" bg-[rgb(41,41,41)] border border-white rounded-[10px] p-8.75 h-50">
          <form bind:this={form} onsubmit={onSubmit}>
               <h3 class="text-[30px] text-[#d9d9d9] pb-4">Offline Account Name</h3>
               <input type="text" class="bg-[#d9d9d9] text-black rounded-[5px] p-1 w-full" placeholder="Name" bind:value={name} required>
               <button type="submit" class="cursor-pointer mt-[15px] border-0 shadow-[0_0_10px_rgba(0,0,0,0.4)] rounded-lg p-[3px] pt-[5px] bg-green-600 text-white leading-[33px] w-full">Save</button>
          </form>
     </div>
</div>