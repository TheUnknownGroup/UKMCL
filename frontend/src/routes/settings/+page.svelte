<script lang="ts">
     import Panel from '#lib/components/Panel.svelte';
     import MainText from '#lib/components/MainText.svelte';
     import { microsoft } from '#lib/ts/settings.ts';
     
     let { children } = $props();
</script>
<div class="flex">
     <Panel />
     <MainText>
          <div class="rounded-md transition-[background] duration-300 ease-in-out light:bg-alice-blue-500/80 bg-alice-blue-900/80 m-1 mr-1.25 p-1 select-none">
               <p class="text-[24px] text-center pr-[45px] text-white">Config</p>
               <div class="transition-[background] duration-300 ease-in-out light:bg-alice-blue-600 bg-alice-blue-950 rounded-sm w-40">
                    <h3 class="transition-[background] duration-300 ease-in-out text-[20px] text-center border-b-alice-blue-700/50 border border-t-0 border-x-0 pt-1 text-white">Login</h3>
                    <button onclick={microsoft} class="transition-[background,color] duration-300 ease-in-out light:text-gray-500 m-1 text-[20px] rounded-sm light:bg-alice-blue-300 bg-alice-blue-600 px-2.5 py-0.5 cursor-pointer">Microsoft Login</button>
               </div>
          </div>
     </MainText>
     {@render children?.()}
</div>