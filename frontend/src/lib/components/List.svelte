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
               <a href="/instances#{name}" class="decoration-[none] text-white text-[17px] font-[550] pl-3.75 border-none bg-transparent cursor-pointer w-full text-left border-t-white border-t">⤷ {name}</a>
          {/each}
     {/if}
</div>