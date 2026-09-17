import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./instCard.js";

const container = document.getElementById("grid-item");
const state = document.getElementById("states");  

export async function loadInstance() {
     container.innerHTML = "";
  try {
    const names = await invoke("get_command");
    if (names.length === 0) {
         container.innerHTML = `<p class="empty state" id="states">No instances yet. <br><br> To create one, press the button above this. <br><br> If you made an instance, it will take some time as the app automatically downloads the assets as soon as the instance is made. </p>`;
      const state = document.getElementById("states");

      function update() {
           if (window.innerHeight > 600) {
                state.style.marginLeft = "33vw";
           } else {
                state.style.marginLeft = "19.2vw";
           }
      }
      window.addEventListener("resize", update);
      update();
      state.classList.remove("hides");
      return;
    }

    for (const name of names) {
      const card = document.createElement("div");
      state.classList.add("hides");
      card.innerHTML = `<inst-card name="${name}"></inst-card>`
      container.appendChild(card);
    }
  } catch (err) {
    console.error(err);
  }
}

listen("instance-removed", () => {
  console.log("received instance-removed event");
  loadInstance();
})

await listen("closing", () => {
     console.log("closed window");
     loadInstance();
})

container.addEventListener('click', async (e) => {
     const btn = e.target.closest('.instance-card');
     if (!btn) return;

     const name = btn.dataset.name;
     const url = `/instance?name=${encodeURIComponent(name)}`;
     const lab = name.replace(/[^a-zA-Z0-9_-]/g, '-')
          .replace(/-+/g, '-')
          .replace(/^-|-$/g, '');
     const label = `instance-${lab}-${Date.now()}`;

     await invoke("spawn_window_2", { label, url, name });
})

loadInstance();