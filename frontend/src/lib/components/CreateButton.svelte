<script lang="ts">
     import { invoke } from '@tauri-apps/api/core';

     async function addInst() {
          try {
               await invoke('spawn_instance_creation_window');
          } catch (e) {
               console.error(e);
          }
     }
</script>

<button class="py-1.75 px-12.5 bg-black/30 rounded-[5px] border border-black ml-1.25 cursor-pointer" onclick={addInst}><img src="/assets/images/plus-square-fill.svg" width="25" alt="Creating" loading="lazy" decoding="async"></button>