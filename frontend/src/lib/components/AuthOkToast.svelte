<script>
    import { onMount } from "svelte";
     import { fly } from "svelte/transition";
     let { finished = $bindable(false) }= $props();

     let translate2 = $state('470%');
     const DURATION = 3000;
     let progress = $state(0);
     let raf;
     let start;

     $effect(() => {
          if (!finished) return;
          progress = 0;
          start = performance.now();
          const tick = (now) => {
               progress = Math.min(100, ((now - start) / DURATION) * 100);
               if (progress < 100) {
                    raf = requestAnimationFrame(tick);
               } else {
                    finished = false;
               }
          };
          raf = requestAnimationFrame(tick);
          return () => cancelAnimationFrame(raf);
     });

     onMount(() => {
          const update = () => {
               translate2 = window.innerHeight > 600 ? '1270%' : '470%';
          }
          window.addEventListener('resize', update);
          update();
     })
</script>

{#if finished}
     <div in:fly={{ y: 10, duration: 150 }} out:fly={{ y: 10, duration: 150 }} class="border border-[#588A58] px-5 py-2 absolute right-1.25 bottom-0 rounded-lg bg-[#356135] text-white select-none overflow-hidden" style:transform={`translateY(${translate2})`}>
          <p>Authenticated</p>
          <div class="absolute left-0 bottom-0 h-[3px] w-full bg-[#588a58]/40">
               <div class="h-full bg-[#8bc34a]" style:width="{progress}%"></div>
          </div>
     </div>
{/if}