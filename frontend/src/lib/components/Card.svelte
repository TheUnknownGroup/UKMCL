<script>
     import { onMount } from 'svelte';
     import List from '$lib/components/List.svelte';
     let { children } = $props();
     
     let minHeight = $state('515px');
     
     onMount(() => {
          const update = () => { 
              minHeight = window.innerHeight > 600 ? '91.7vh' : '515px'; 
          };
          update();
          window.addEventListener('resize', update);
          return () => window.removeEventListener('resize', update);
     })
</script>

<style>
     h3 {
          color: white;
          font-size: 20px;
          padding-left: 10px;
          padding-top: 8px;
     }
     p {
          color: white;
          font-size: 16px;
          padding-right: 2px;
          overflow: auto;
          padding-left: 15px;
          padding-bottom: 5px;
     }
     .color {
          border-color: #A8A8A8;
     }
</style>
<div class="flex justify-center pr-1.25 ml-1.25">
     <div class="border color rounded-lg bg-[rgba(0,0,0,0.4)] w-full mt-1.25 select-none">
          <h3>Welcome!</h3>
          <p>This is our launcher, we hope you enjoy!</p>
     </div>
</div>
<div class="flex justify-center pr-1.25 ml-1.25 float-left">
     <div class="border color rounded-lg bg-[rgba(0,0,0,0.4)] w-27 mt-1.25 pb-2.5 select-none" style:min-height={minHeight}>
          <h3>Latest Instances</h3>
          <List />
     </div>
</div> 
<div class="flex justify-center pr-1.25 ml-1.25">
     <div class="border color rounded-lg w-full mt-1.25 bg-[rgba(0,0,0,0.4)] pb-2.5 overflow-y-auto select-none" style:min-height={minHeight}>
          <h3 class="border-b color pb-1.25">Latest News</h3>
     </div>
</div>

{@render children?.()}

