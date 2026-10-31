<script lang="ts">
     import { onMount } from "svelte";
     import { listen } from '@tauri-apps/api/event';
     import { fly } from 'svelte/transition';
     import { toasts, pushToast, pauseToast, resumeToast, type ToastKind } from "$lib/toast.svelte";
     
     const colors: Record<ToastKind, string> = {
          success: 'bg-green-600',
          error: 'bg-red-600',
          info: 'bg-blue-600',
     };

     const subColors: Record<ToastKind, string> = {
          success: 'bg-green-400/40',
          error: 'bg-red-400/40',
          info: 'bg-blue-400/40',
     };

     const fills: Record<ToastKind, string> = {
          success: 'bg-green-200',
          error: 'bg-red-200',
          info: 'bg-blue-200',
     };

     onMount(() => {
          let unlisten: (() => void) | undefined;
          let cancelled = false;

          listen<{ kind: ToastKind; message: string }>('toast', (e) =>
               pushToast(e.payload.kind, e.payload.message)
          ).then((fn) => (cancelled ? fn(): (unlisten = fn)));
          
          return () => {
               cancelled = true;
               unlisten?.();
          }
     });
</script>

<div class="fixed bottom-2 right-2 z-50 flex flex-col gap-2 pointer-events-auto select-none">
     {#each toasts as t (t.id)}
          <div in:fly={{ y: 10, duration: 300 }} out:fly={{ y: 10, duration: 300 }} onmouseenter={() => pauseToast(t.id)} onmouseleave={() => resumeToast(t.id)} role="status" class="{colors[t.kind]} text-white relative overflow-hidden px-4 py-2 rounded-md shadow-lg max-w-[320px]">
               {t.message}
               <div class="absolute inset-x-0 bottom-0 h-[3px] {subColors[t.kind]}">
                    <div class="h-full origin-left animate-fill {fills[t.kind]}" style:animation-play-state={t.paused ? 'paused' : 'running'}></div>
               </div>
          </div>
     {/each}
</div>