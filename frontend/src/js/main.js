// import { invoke } from "@tauri-apps/api/core";

const home_btn = document.getElementById("home");
home_btn.href = "./"

const logo_btn = document.getElementById("logo");
logo_btn.href = "https://github.com/TheUnknownGroup";
logo_btn.target = "_blank";

const inst_btn = document.getElementById("instances");
inst_btn.href = "/instances"

// function test2() {
//      invoke("test");
// }

// test2()
const form = document.getElementById("form");
const add_inst = document.getElementById("add_new_inst");
add_inst.addEventListener("click", () => {
     form.classList.add("hidden");
})

const cancel_btn = document.getElementById("cancel-btn");
cancel_btn.addEventListener("click", () => {
     form.classList.remove("hidden");
})