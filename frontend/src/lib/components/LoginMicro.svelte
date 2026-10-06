<script>
     import { onMount, onDestroy } from 'svelte';
     import { resolveAuth, createCount, format, copys } from '$lib/js/login.js';
     import { invoke } from "@tauri-apps/api/core";
     import { listen } from "@tauri-apps/api/event";

     let open = $state(false);
     let visible = $state(false);
     let link = $state('');
     let code = $state('');
     let rem = $state(0);

     let stops = null;
     let visibleRaf = null;

     async function show() {
          let data;
          try {
               data = await invoke("auths");
          } catch (e) {
               console.error(e);
               resolveAuth(null);
               return;
          }

          link = data.verification_uri;
          code = data.user_code;
          open = true;

          stops?.();
          stops = createCount(data.expires_in, (r) => { rem = r; }, () => hide(null));
     }

     function hide(result) {
          stops?.();
          stops = null;
          open = false;
          visible = false;
          resolveAuth(result);
     }

     async function cancel() {
          await invoke("cancel");
     }

     $effect(() => {
          if (open) {
               visibleRaf = requestAnimationFrame(() => { visible = true});
               return () => { if (visibleRaf) cancelAnimationFrame(visibleRaf); };
          } else {
               visible = false;
          }
     });

     onMount(async (e) => {
          const unlisteners = [];
          listen('open_msa', () => show()).then((fn) => unlisteners.push(fn));
          listen('close_msa', () => hide('done')).then((fn) => unlisteners.push(fn));
          return () => unlisteners.forEach((fn) => fn());
     });

     onDestroy(() => {
          stops?.();
     });
</script>

{#if open}
<div class="fixed inset-0 z-50 bg-black/50 flex items-center justify-center transition-[opacity,visibility] ease-in-out duration-300 {visible ? 'opacity-100 visible' : 'opacity-0 invisible' }" >
     <div class="bg-[#2e302f] border-white border rounded-lg pt-2.5 px-11.25 pb-7 text-center text-white hover:shadow-[0_0_10px_white] transition-shadow duration-300 ease-in-out">
          <h3 class="text-[24px] font-semibold border-2 border-transparent pt-3.5 pb-3.5"> Microsoft Auth </h3>
          <div class="text-left">
               <p class="mb-[1.1rem] text-[18px]">Verification link: {link}</p>
               <p class="mb-[1.1rem] text-[18px]">Code: {code}</p>
               <p class="mb-[1.1rem] text-[18px]">Time left: <span>{format(rem)}</span></p>
          </div>
          <div class="flex justify-center gap-3">
               <button onclick={() => { hide(null); cancel(); }} class="p-1.25 pt-[8px] pb-[5px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] cursor-pointer text-white text-[15px] bg-[#dc2626] hover:bg-[#F26363]">Cancel</button>
               <a href={link} target="_blank" class="p-1.25 pt-[8px] pb-[5px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] cursor-pointer text-white text-[15px] bg-[#238510] hover:bg-[#3B9E2C]">Auth Me</a>
               <button onclick={() => copys(code)} target="_blank" class="p-1.25 pt-[8px] pb-[5px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] cursor-pointer text-white text-[15px] bg-[#297999] hover:bg-[#3AAFDE]">Copy Code</button>
          </div>
     </div>
</div>
{/if}