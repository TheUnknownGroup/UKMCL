<script lang="ts">
     import Panel from '$lib/components/Panel.svelte';
     import MainText from '$lib/components/MainText.svelte';
     import { microsoft } from '$lib/ts/settings';
     
     let { children } = $props();
</script>
<div class="flex">
     <Panel />
     <MainText>
          <div class="rounded-md bg-alice-blue-950/50 m-1 mr-1.25 p-1 select-none">
               <p class="text-[24px] text-center pr-[45px]">Config</p>
               <div class="bg-alice-blue-950 rounded-sm w-40">
                    <h3 class="text-[20px] text-center border-b-alice-blue-800/50 border border-t-0 border-x-0 pt-1">Login</h3>
                    <button onclick={microsoft} class="m-1 text-[20px] rounded-sm bg-alice-blue-600 px-2.5 py-0.5 cursor-pointer">Microsoft Login</button>
               </div>
          </div>
     </MainText>
     {@render children?.()}
</div>