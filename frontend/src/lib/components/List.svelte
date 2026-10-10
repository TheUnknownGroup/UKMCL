<script>
    import { invoke } from '@tauri-apps/api/core';
     import { onMount } from 'svelte';
     
     let names = $state([]);
     let error = $state(null);
     let loading = $state(true);

     async function getLatest() {
          loading = true;
          error = null;
          try {
               names = await invoke('get_command');
          } catch (err) {
               error = err;
               console.error(err);
          } finally {
               loading = false
          }
     }

     async function setup(name) {
          const url = `/instance?name=${encodeURIComponent(name)}`;
          const lab = name.replace(/[^a-zA-Z0-9_-]/g, '-').replace(/-+/g, '-').replace(/^-|-$/g, '');
          const label = `instance-${lab}-${Date.now()}`;
          await invoke('spawn_window_2', { label, url, name });
     }
     
     onMount(getLatest);
</script>

<style>
     p {
          color: white;
          font-size: 16px;
          padding-right: 5px;
          overflow: auto;
          padding-left: 15px;
          padding-bottom: 5px;
     }
</style>

<div id="list">
     {#if loading}
          <p class="pr-1.5">No instances yet.</p>
     {:else if error}
          <p>Failed to load instances.</p>
     {:else if names.length === 0}
          <p class="pr-1.5">No instances yet.</p>
     {:else}
          {#each names as name}
               <div><button onclick={() => setup(name)} class="cursor-pointer w-full block whitespace-nowrap overflow-hidden text-left text-ellipsis pl-2.25 right-0 no-underline text-white text-[17px] hover:bg-black/30 transition duration-200 ease-in-out">➙ {name}</button></div>
          {/each}
     {/if}
</div>