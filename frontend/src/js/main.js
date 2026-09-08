import { invoke } from "@tauri-apps/api/core";

const home_btn = document.getElementById("home");
home_btn.href = "./"

const logo_btn = document.getElementById("logo");
logo_btn.href = "https://github.com/TheUnknownGroup";
logo_btn.target = "_blank";

const inst_btn = document.getElementById("instances");
inst_btn.href = "/instances"

const set_btn = document.getElementById("settings");
set_btn.href = "/settings"

export async function getLatestInst() {
     const list = document.getElementById("list");
     list.innerHTML = ""
     try {
          const names = await invoke("get_command");
          if (names.length === 0) {
               list.innerHTML = `<p> No latest instance made. </p>`
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