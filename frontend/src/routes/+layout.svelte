<script lang="ts">
     import "../app.css";
     import TitleBar from '#lib/components/TitleBar.svelte';
     import GeneralToast from "#lib/components/GeneralToast.svelte";
     import LoginMicrosoft from "#lib/components/LoginMicrosoft.svelte";
     import { getCurrentWindow } from "@tauri-apps/api/window";
     import { onMount } from "svelte";
     const win = getCurrentWindow();
     let maxed = $state(false);
     let focused = $state(true);
     onMount(() => {
          if (win.label == "main") {
               let unlistenResize: (() => void) | undefined;
               let unlistenFocus: (() => void) | undefined;
               let cancelled = false;
               const sync = async () => {
                    maxed = await win.isMaximized();
               };
               sync();
               win.isFocused().then((f) => (focused = f));
               win.onResized(sync).then((fn) => (cancelled ? fn() : (unlistenResize = fn)));
               win.onFocusChanged(({ payload }) => (focused = payload)).then((fn) => (cancelled ? fn() : (unlistenFocus = fn)));
               return () => {
                    cancelled = true;
                    unlistenResize?.();
                    unlistenFocus?.();
               }
          }
     });
     
     let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href="/assets/icon.png" />
</svelte:head>
<div class="h-screen {maxed ? '' : 'p-2'}">
     <div tabindex="-1" class="app relative h-full isolate contain-paint outline-none transition-shadow duration-200 overflow-hidden {maxed ? '' : 'rounded-[10px]'} {!maxed && focused ? 'shadow-[0_0_10px_rgba(0,0,0,0.55)]' : ''}">
          <TitleBar/>
          <GeneralToast />
          <LoginMicrosoft/>
          {@render children?.()}
     </div>
</div>

