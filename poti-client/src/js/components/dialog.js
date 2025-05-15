let dialog_parent;
let dialog_info;

let callback = undefined;

function show_dialog(text, callback_fn) {
  dialog_parent.style.display = "inherit";
  dialog_info.innerText = text;

  callback = callback_fn;
}
function hide_dialog() {
  dialog_parent.style.display = "none";
  dialog_info.innerText = "";
}

function confirm() {
  hide_dialog();

  if (callback == undefined) {
    return;
  }
  callback(true);
  callback = undefined;
}
function cancel() {
  hide_dialog();

  if (callback == undefined) {
    return;
  }

  callback(false);
  callback = undefined;
}

document.addEventListener("DOMContentLoaded", async () => {
  dialog_parent = document.getElementById("dialog-parent");
  dialog_info = document.getElementById("dialog-info");

  document
    .getElementById("dialog-close")
    .addEventListener("click", () => cancel());
  document
    .getElementById("dialog-confirm")
    .addEventListener("click", () => confirm());
});

export { show_dialog };
