import { invoke } from "@tauri-apps/api/core";

export async function getLatestInst() {
     const list = document.getElementById("list");
     list.innerHTML = ""
     try {
          const names = await invoke("get_command");
          if (names.length === 0) {
               list.innerHTML = `<p class="empty"> No latest instance made. </p>`
          }

          for (const name of names) {
               const newName = document.createElement("button");
               newName.setAttribute("onclick", `location.href="/instances#${name}"`);
               newName.innerText = `⤷ ${name}`;
               list.appendChild(newName);
          }
     } catch (err) {
          console.error(err);
     }
}

getLatestInst();

const insts = document.getElementById("insts");
const mainContent = document.getElementById("main_content");

function update() {
     if (window.innerHeight > 600) {
          insts.style.minHeight = "89vh";
          mainContent.style.minHeight = "89vh";
     } else {
          insts.style.minHeight = "465px";
          mainContent.style.minHeight = "465px";
     }
}

update();

window.addEventListener("resize", update);