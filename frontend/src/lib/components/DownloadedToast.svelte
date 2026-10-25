<script>
     import { onMount } from "svelte";
     import { fly } from "svelte/transition";
     let { downloads = $bindable(false), name = $bindable('') } = $props();
     const DURATION = 3000;
     let progress = $state(0);
     let raf;
     let start;

     $effect(() => {
          if (!downloads) return;
          progress = 0;
          start = performance.now();
          const tick = (now) => {
               progress = Math.min(100, ((now - start) / DURATION) * 100);
               if (progress < 100) {
                    raf = requestAnimationFrame(tick);
               } else {
                    downloads = false;
                    name = '';
               }
          };

          raf = requestAnimationFrame(tick);
          return () => cancelAnimationFrame(raf);
     });
</script>

{#if downloads}
     <div in:fly={{ y: 10, duration: 150 }} out:fly={{ y: 10, duration: 150 }} class="border border-[#588a58] px-1.5 py-2 absolute right-5 bottom-15 w-[20%] rounded-lg bg-[#356135] text-white select-none">
          <p class="flex justify-center">Downloading...</p>
          <p class="text-[#99CC99] text-[17px] text-ellipsis overflow-hidden whitespace-nowrap">Downloaded to iohgoiwhgwpgjqpogj</p>
          <div class="absolute left-0 bottom-0 h-[3px] w-full bg-[#588a58]/40">
               <div class="h-full bg-[#8bc34a]" style:width="{progress}%"></div>
          </div>
     </div>
{/if}