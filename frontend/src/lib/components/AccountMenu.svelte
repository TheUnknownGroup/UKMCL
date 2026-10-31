<script lang="ts">
     import { goto } from "$app/navigation";
     import { account, microsoft, signOut } from "$lib/ts/settings";
     import { onMount } from "svelte";
     import { slide } from "svelte/transition";
     const SETTINGS = `/settings?title=${encodeURIComponent("Settings")}`
     
     let open = $state(false);
     let switching = $state(false);
     let name = $state('');
     let uuid = $state('');
     let isMsa = $state(false);
     let imgFailed = $state(false);
     let url = $derived(uuid ? `https://mc-heads.net/avatar/${uuid}/100` : '');
     let showAvatar = $derived(isMsa && !!url && !imgFailed);
     let root: HTMLDivElement;
     const items = [
          { label: 'Profile', action: () => { goto(SETTINGS) }},
          { label: 'Switch Account', action: () => (switching = !switching), keepOpen: true },
          { label: 'Sign Out', action: () => { signOut() }},
     ];

     function pick(item: (typeof items)[number]) {
          item.action();
          if (!item.keepOpen) open = false;
     }

     async function load() {
          try {
               const acc: any = await account();
               if (!acc) return;
               name = acc.username ?? '';
               uuid = acc.uuid ?? '';
               isMsa = acc.user_type === "msa";
               imgFailed = false;
          } catch (e) {
               console.error("failed to load account: ", e);
          }
     }

     function toggle() {
          open = !open;
          if (open) load();
     }

     $effect(() => {
          if (!open) switching = false;
     })

     onMount(load);
</script>

<svelte:window onclick={(e) => {if (open && !root.contains(e.target as Node)) open = false; }}
     onkeydown={(e) => {if (e.key === 'Escape') open = false; }}
     />

<div bind:this={root} class="fixed bottom-12.25 w-[50px]">
     {#if open}
          <div class="absolute bottom-0 left-full ml-2 flex items-end gap-2">
               <div id="account-menu" transition:slide={{ duration: 200 }} class="w-[160px] overflow-hidden rounded-md bg-alice-blue-950/95 shadow-lg">
                    <button class="flex items-center w-full px-3 py-2 text-left cursor-pointer hover:bg-white/10">
                         <img src={showAvatar ? url : '/assets/images/icon.png'} alt="Account" loading="lazy" decoding="async" width="25" class="rounded-sm mr-[5px]">{name}
                    </button>
                    {#each items as i}
                         <button onclick={() => pick(i)} class="block w-full px-3 py-2 text-left cursor-pointer hover:bg-white/10">
                              {i.label}
                         </button>
                    {/each}
               </div>
               <div inert={!switching} class="mb-10 w-[130px] overflow-hidden rounded-md bg-alice-blue-800 shadow-lg transition-[opacity,translate] duration-200 ease-in-out {switching ? 'translate-x-0 opacity-100' : '-translate-x-3 opacity-0 pointer-events-none'}">
                    <button class="block w-full whitespace-nowrap text-left px-2 py-2 cursor-pointer hover:bg-white/10">Offline</button>
                    <button class="block w-full whitespace-nowrap text-left px-2 py-2 cursor-pointer hover:bg-white/10" onclick={() => { open = false; microsoft(); }}>Microsoft</button>
               </div>
          </div>
     {/if}
     <button onclick={toggle} aria-expanded={open} aria-controls="account-menu" class="flex justify-center py-0.5 w-full cursor-pointer">
          <img src={showAvatar ? url : '/assets/images/icon.png' } alt="Account" width="38" class="rounded-sm" onerror={() => { if (showAvatar) imgFailed = true; }}>
     </button>
</div>