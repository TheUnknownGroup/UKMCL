<script lang="ts">
     import { setup } from "$lib/ts/instance";
     import { onMount } from "svelte";
     import CreateButton from "./CreateButton.svelte";
     let container: any;
     let isEmpty = $state(false);
     onMount(() => {
          let cleanup: any;
          setup(container, (empty: any) => {
               isEmpty = empty;
          }).then((dispose) => { cleanup = dispose; }).catch((e) => console.error(e));
          return () => cleanup?.();
     })
</script>

<div class="p-[0.3125rem_0] justify-start">
     <CreateButton />
     <div class="grid pt-1.25 gap-1.25 grid-cols-[repeat(auto-fit,minmax(196px,198px))]" bind:this={container}></div>
</div>