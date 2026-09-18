<script>
     import { onMount } from 'svelte';
     import { downloadSetup } from '$lib/js/download.js';

     let pct = $state(0);
     let labelT = $state('');
     let etaTxt = $state('');

     onMount(() => downloadSetup({
          onProgress: ({ pct: p, label, eta }) => {
               pct = p;
               labelT = label;
               etaTxt = eta;
          },
          onComplete: () => { pct = 100; etaTxt = '';},
     }));
</script>

<style>
     :root {
          background: none;
          background-color: #303030;
     }
     :root:before {
          opacity: 0;
     }
</style>

<div class="fixed inset-0 z-50 flex bg-[#1a1a1a] h-6.25 top-[43%] border border-black mx-1.25">
     <div class="h-full w-[0%] bg-[#4caf50] transition-[width] duration-300 ease-in-out" style:width="{pct}%"></div>
     <span class="absolute left-2 text-[12px] leading-5 text-white whitespace-nowrap flex items-center justify-center translate-y-[-130%] border-white border bg-[rgba(0,0,0,0.3)] px-1.25 pt-1.25 pb-0.75 select-none">{labelT}</span>
     <span class="absolute translate-y-[150%] right-0 leading-5 text-[12px] text-[#ccc] select-none">{etaTxt}</span>
</div>