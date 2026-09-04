import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { confirmDialog } from "./confirmDialog.js";

const container = document.getElementById("grid-item");

// async function getVers() {
//      const list = document.getElementById("ver-list");
     
//      try {
//           await invoke("load_versions");
//           const ids = await invoke("get_ver_id");
//           list.innerHTML = "";
//           for (const id of ids) {
//                const option = document.createElement("option");
//                option.value = id;
//                option.textContent = id;
//                list.appendChild(option);
//           }
//      } catch (err) {
//           console.error("Failed to load version list: ", err);
//      }
// }

container.addEventListener("click", async (e) => {
  const btn = e.target.closest(".delete-btn");
  if (!btn) return;

  const name = btn.dataset.name;
  const confirmed = await confirmDialog(`Delete instance "${name}"?`);
  if (!confirmed) return;

  try {
    await invoke("delete_command", { instanceName: name });
  } catch (err) {
    alert(`Failed to delete instance "${name}": ${err}`)
  }
});

async function loadInstance() {
     container.innerHTML = "";

  try {
    const names = await invoke("get_command");
    if (names.length === 0) {
      container.innerHTML = `<p class="empty-state">No instances yet.</p>`;
      return;
    }

    for (const name of names) {
      const card = document.createElement("div");
      card.className = "instance-card";
      card.innerHTML = `
        <h3>${name}</h3>
        <div class="card-act">
          <button class="launch-btn btn2" data-name="${name}"><img src="/assets/images/play.svg" alt="Launch"></button>
          <button class="delete-btn btn2" data-name="${name}"><img src="/assets/images/trash.svg" alt="Delete"></button>
          <button class="edit-btn btn2" data-name="${name}"><img src="/assets/images/tools.svg" alt="Edit"></button>
        </div>
        `
      container.appendChild(card);
    }
  } catch (err) {
    console.error(err);
  }
}

/** @type {HTMLFormElement} **/
const form = document.getElementById("inst_creation");

listen("instance-removed", () => {
  console.log("received instance-removed event");
  loadInstance();
})

form.addEventListener("submit", async (e) => {
    e.preventDefault();
    /** @type {HTMLInputElement} **/
    const input = document.getElementById("instance");
    // const input_1 = document.getElementById("ver-list");
    const instanceName = input.value.trim();
    // const instanceVersion = input_1.value;
    
    try {
         const path = await invoke("create_command", { instName: instanceName });
         form.reset();
         loadInstance();
         
    } catch (err) {
        console.error(err);
    }
})

loadInstance();
// getVers();