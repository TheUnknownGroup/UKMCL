<script>
     import { invoke } from "@tauri-apps/api/core";
     async function addInst() {
          try {
               await invoke('spawn_window');
          } catch (err) {
               console.error(err);
          }
     }
     
     let { children } = $props();
</script>

<button class="py-1.75 px-12.5 bg-black/30 rounded-[5px] border-2 border-black ml-1.25 cursor-pointer" onclick={addInst}><img width="25" src="/assets/images/plus-square-fill.svg" alt="Creating"></button>
{@render children?.()}