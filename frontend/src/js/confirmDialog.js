export function confirmDialog(message) {
     return new Promise((re) => {
          const modal = document.getElementById("modal");
          const msg = document.getElementById("confirm-msg");
          const ok = document.getElementById("ok");
          const cancel = document.getElementById("cancel");

          msg.textContent = message;
          modal.classList.add("hidden");
          
          function clear(result) {
               modal.classList.remove("hidden");
               ok.removeEventListener("click", onOk);
               cancel.removeEventListener("click", onCancel);
               re(result)
          }

          function onOk() { clear(true); }
          function onCancel() { clear(false); }

          ok.addEventListener("click", onOk);
          cancel.addEventListener("click", onCancel);
     });
}