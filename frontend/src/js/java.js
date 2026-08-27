import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { confirmDialog } from "./confirmDialog.js";

const container = document.getElementById("instance-list");

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
const form = document.getElementById("test");

listen("instance-removed", () => {
  console.log("received instance-removed event");
  loadInstance();
})

form.addEventListener("submit", async (e) => {
    e.preventDefault();
    /** @type {HTMLInputElement} **/
    const input = document.getElementById("instance");
    const instanceName = input.value.trim();
    try {
        const path = await invoke("create_command", { instanceName: instanceName });
        form.reset();
        loadInstance();
    } catch (err) {
        console.error(err);
    }
})

loadInstance();