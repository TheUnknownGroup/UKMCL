import { invoke } from "@tauri-apps/api/core";
import { confirmDialog } from "./confirmDialog";
import { getCurrentWindow } from "@tauri-apps/api/window";

const params = new URLSearchParams(window.location.search);
const names = params.get('name') || 'Unknown';

const container = document.getElementById("card");

document.title = names;
document.getElementById('name').textContent = names;

container.innerHTML = `
     <button class="launch-btn btn2" data-name="${names}"><img src="/assets/images/play.svg" alt="Launch"></button>
     <button class="delete-btn btn2" data-name="${names}"><img src="/assets/images/trash.svg" alt="Delete"></button>
     <button class="edit-btn btn2" data-name="${names}"><img src="/assets/images/tools.svg" alt="Edit"></button>
`

container.addEventListener("click", async (e) => {
  const btn = e.target.closest(".delete-btn");
  if (!btn) return;
  
  const name = btn.dataset.name;
  const confirmed = await confirmDialog(`Delete instance "${name}"?`);
  if (!confirmed) return;
     
  try {
       await invoke("delete_command", { instName: name });
  } catch (err) {
    alert(`Failed to delete instance "${name}": ${err}`)
  }

     setTimeout(() => getCurrentWindow().close(), 600);
});

container.addEventListener("click", async (e) => {
  const btn = e.target.closest(".launch-btn");
  if (!btn) return;
  const name = btn.dataset.name;
  try {
       await invoke("launch_command", { instName: name });
  } catch (err) {
       alert(`Failed to launch: "${name}": ${err}`)
  }
}); 