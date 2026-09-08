const form = document.getElementById("form");
const add_inst = document.getElementById("add_new_inst");
add_inst.addEventListener("click", () => {
     form.classList.add("hidden");
})

const cancel_btn = document.getElementById("cancel-btn");
cancel_btn.addEventListener("click", () => {
     form.classList.remove("hidden");
})

form.addEventListener("submit", () => {
     form.classList.remove("hidden");
})
