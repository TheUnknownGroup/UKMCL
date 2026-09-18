<script>
     import { onMount } from 'svelte';
     import { resolve } from '$lib/js/confirmDialog.js';

     let open = $state(false);
     let visible = $state(false);
     let message = $state('');

     $effect(() => {
          if (open) {
               const id = requestAnimationFrame(() => { visible = true; });
               return () => cancelAnimationFrame(id); 
          } else {
               visible = false;
          }
     })

     onMount(() => {
          const handler = (e) => {
               message = e.detail.message;
               open = true;
          };
          window.addEventListener('open-confirm', handler);
          return () => window.removeEventListener('open-confirm', handler);
     });

     function ok() { open = false; resolve(true); }
     function cancel() { open = false; resolve(false); }
</script>

{#if open}
     <div class="fixed inset-0 z-50 bg-black/50 flex items-center justify-center transition-[opacity,visibility] ease-in-out duration-300 {visible ? 'opacity-100 visible' : 'opacity-0 invisible'}" >
          <div class="bg-[#2e302f] border-white border rounded-lg pt-2.5 px-11.25 pb-7 text-center text-white hover:shadow-[0_0_10px_white] transition-shadow duration-300 ease-in-out">
               <h3 class="text-[24px] font-semibold border-2 border-transparent pt-3.5 pb-3.5"> Confirm </h3>
               <p class="mb-[1.1rem] text-[18px]">{message}</p>
               <div class="flex justify-center gap-3">
                    <button class="p-1.25 pt-[8px] pb-[5px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] cursor-pointer text-white text-[15px] bg-[#36373b] hover:bg-[#4b5563]" onclick={cancel}>Cancel</button>
                    <button class="p-1.25 pt-[8px] pb-[5px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] cursor-pointer text-white text-[15px] bg-[#dc2626] hover:bg-[#ef4444]" onclick={ok}>Delete</button>
               </div>
          </div>
     </div>
{/if}