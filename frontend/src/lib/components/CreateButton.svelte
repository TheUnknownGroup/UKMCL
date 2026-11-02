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

<button class="px-10 pt-1.25 pb-0.75 bg-black/30 rounded-[5px] border border-black ml-1.25 cursor-pointer text-white" onclick={addInst} aria-label="Create"><i class="bi bi-plus-square-fill text-[15px]"></i></button>