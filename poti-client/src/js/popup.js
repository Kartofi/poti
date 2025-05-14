let popup_parent;
let popup_info;
let popup_close;

function show_popup(text) {
  popup_parent.style.display = "inherit";
  popup_info.innerText = text;
}
function hide_popup() {
  popup_parent.style.display = "none";
  popup_info.innerText = "";
}

document.addEventListener("DOMContentLoaded", async () => {
  popup_parent = document.getElementById("popup-parent");
  popup_close = document.getElementById("popup-close");
  popup_info = document.getElementById("popup-info");

  document
    .getElementById("popup-close")
    .addEventListener("click", () => hide_popup());
});

export { show_popup };
