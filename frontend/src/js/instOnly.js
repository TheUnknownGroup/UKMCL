import { invoke } from "@tauri-apps/api/core";

const add_inst = document.getElementById("add_new_inst");
add_inst.addEventListener("click", async () => {
     await invoke("spawn_window");
})
