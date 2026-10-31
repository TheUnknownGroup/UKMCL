<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { resolveAuth, createCount, format, copy, cancel } from "#lib/ts/settings";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    let open = $state(false);
    let visible = $state(false);
    let link = $state('');
    let code = $state('');
    let rem = $state(0);
    let stops: any = null;
    let visibleRef: any = null;

    async function show() {
         let data: any;
         try {
              data = await invoke("get_auth_info");
         } catch (e) {
              console.error(e);
              resolveAuth(null);
              return;
         }

         link = data.verification_uri;
         code = data.user_code;
         open = true;
         stops?.();
         stops = createCount(data.expires_in, (r: any) => { rem = r; }, () => hide(null));
    }

    function hide(result: any) {
         stops?.();
         stops = null;
         open = false;
         visible = false;
         resolveAuth(result);
    }

    $effect(() => {
         if (open) {
              visibleRef = requestAnimationFrame(() => {visible = true});
              return () => { if (visibleRef) cancelAnimationFrame(visibleRef); };
         } else {
              visible = false;
         }
    });

    onMount(() => {
         const unlisteners: any = [];
         let cancelled = false;
         const add = (p: Promise<() => void>) => p.then((fn) => (cancelled ? fn() : unlisteners.push(fn)));
         add(listen('open_msa', () => show()));
         add(listen('close_msa', () => hide('done')))
         return () => {
              cancelled = true;
              unlisteners.forEach((fn: any) => fn());
              stops?.();
         }
    });

    onDestroy(() => {
         stops?.();
    });
</script>

{#if open}
     <div class="fixed inset-0 z-50 bg-black/50 flex items-center justify-center transition-[opacity,visiblility] ease-in-out duration-300 {visible ? 'opacity-100 visible' : 'opacity-0 invisible'}">
          <div class="bg-creation-3 border-white border rounded-lg pt-2.5 px-11.25 pb-7 text-center text-white hover:shadow-[0_0_10px_white] transition-shadow duration-300 ease-in-out">
               <h3 class="text-[24px] font-semibold border-2 border-transparent pt-3.5 pb-3.5">Microsoft Auth</h3>
               <div class="text-left">
                    <p class="mb-[1.1rem]">Verification link: {link}</p>
                    <p class="mb-[1.1rem]">Code: {code}</p>
                    <p class="mb-[1.1rem]">Time left: <span>{format(rem)}</span></p>
               </div>
               <div class="flex justify-center gap-3">
                    <button class="p-1.25 pt-[8px] pb-[5px] rounded-[5px] shadow-[0_0_10px] shadow-black/40 cursor-pointer text-white text-[15px] bg-[#dc2626] hover:bg-[#f26363]" onclick={() => {hide(null); cancel(); }}>Cancel</button>
                    <a href={link} target="_blank" class="p-1.25 pt-[8px] pb-[5px] rounded-[5px] shadow-[0_0_10px] shadow-black/40 cursor-pointer text-white text-[15px] bg-[#238510] hover:bg-[#3b9e2c]">Auth Me</a>
                    <button class="p-1.25 pt-[8px] pb-[5px] rounded-[5px] shadow-[0_0_10px] shadow-black/40 cursor-pointer text-white text-[15px] bg-[#297999] hover:bg-[#3aafde]" onclick={() => copy(code)}>Copy Code</button>
               </div>
          </div>
     </div>
{/if}