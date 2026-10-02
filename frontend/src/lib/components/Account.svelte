<script>
     import { acc } from "$lib/js/account.js";
     import { onMount } from "svelte";

     let placeholder = $state(false);
     let name = $state('');

     onMount(async (e) => {
          let accInfo = await acc();

          name = accInfo.username;

          if (accInfo.user_type === "legacy") {
               placeholder = true
          } 
     });
     
     let { children } = $props();
</script>

<div class="h-14 border border-white rounded-lg pr-2 ml-1.25 mt-1.75 bottom-0 mr-1.25 bg-black/40 flex">
     {#if placeholder == true}
          <div class="text-white pt-1.75 pl-1.75">
               <img src="/assets/images/icon.png" width="50" alt="Account Face"/>
          </div>  
     {/if}
     <a href="/settings"><p class="flex text-white pt-[17px] pl-[25px] text-[25px] font-medium">{name}</p></a>
</div>

{@render children?.()}