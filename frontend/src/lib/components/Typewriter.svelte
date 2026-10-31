<script lang="ts">
     let { text, speed = 50 } = $props();
     let shown = $state('');

     $effect(() => {
          shown = '';
          let i = 0;
          const id = setInterval(() => {
               shown = text.slice(0, ++i);
               if (i >= text.length) clearInterval(id);
          }, speed);
          return () => clearInterval(id);
     })
</script>

<span class="inline-grid pointer-events-none" aria-label={text}>
     <span class="invisible col-start-1 row-start-1" aria-hidden="true">{text}</span>
     <span class="col-start-1 row-start-1 whitespace-nowrap" aria-hidden="true">{shown}<span class="inline-block w-px h-[1.17em] ml-1.5 align-text-bottom bg-current animate-blink"></span></span>
</span>