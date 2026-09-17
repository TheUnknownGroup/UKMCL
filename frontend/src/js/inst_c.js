import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from "@tauri-apps/api/core";

const forms = document.getElementById("inst_creation");

const form = document.getElementById("form");
const cancel_btn = document.getElementById("cancel-btn");
cancel_btn.addEventListener("click", () => {
     setTimeout(() => getCurrentWindow().close(), 800);
})

async function getVers() {
     const list = document.getElementById("ver-list");

     try {
          await invoke("load_versions");
          const ids = await invoke("get");
          list.innerHTML = "";
          for (const id of ids) {
               const option = document.createElement("option");
               option.value = id;
               option.textContent = id;
               list.appendChild(option);
          }
     } catch (err) {
          console.error("Failed to load version list: ", err);
     }
}

form.addEventListener("submit", async (e) => {
     e.preventDefault();
     /** @type {HTMLInputElement} **/
     const input = document.getElementById("instance");
     const input_1 = document.getElementById("ver-list");
     const instanceName = input.value.trim();
     const instanceVersion = input_1.value;
 
     try {
          await invoke("create_command", { instName: instanceName, ver: instanceVersion });
          forms.reset();
     } catch (err) {
         console.error(err);
     }
     
     setTimeout(() => getCurrentWindow().close(), 300);
})

getVers();