const sign = document.getElementById("y");
const log = document.getElementById("n");
// @ts-ignore
sign.addEventListener("click", ()=> {
    location.href = "./src/html/login.html";
});
// @ts-ignore
log.addEventListener("click", () => {
    location.href = "./src/html/signup.html";
})