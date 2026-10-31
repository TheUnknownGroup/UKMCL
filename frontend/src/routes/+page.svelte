<script lang="ts">
     import Panel from '#lib/components/Panel.svelte';
     import MainText from '#lib/components/MainText.svelte';
     import Typewriter from '#lib/components/Typewriter.svelte';
     import { onMount } from 'svelte';
     import { invoke } from '@tauri-apps/api/core';
     import { listen } from '@tauri-apps/api/event';
     let name = $state("");
     async function load_info() {
          try {
               await listen('authed', () => location.reload());
               await invoke('write_initial').catch((e) => console.error(e));
               const info: any = await invoke('account_info');
               name = info.username ?? '';
          } catch (e) {
               console.error(e);
          }
     }
     onMount(load_info);
     let { children } = $props(); 
</script>
<div class="flex">
     <Panel />
     <MainText>
          <div class="select-none bg-alice-blue-950/80 pt-1.25 pr-[45px] text-center m-1 rounded-sm text-[23px]">
               <Typewriter text={name ? `Welcome, ${name}!`: 'Welcome!'} speed={100} />
          </div>
     </MainText>
     {@render children?.()}
</div>
