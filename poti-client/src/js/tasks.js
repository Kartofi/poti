const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const appWindow = window.__TAURI__.window;

let element_html = `<a id="[id]-name">[name]</a>
        <a id="[id]-downloaded">[downloaded]</a>
        <a id="[id]-size">[size]</a>
        <a id="[id]-speed">[speed]</a>
        <br>
        <progress max="100" value="[progress]" id="[id]-progress"></progress>
      `;
const sizes = ["bytes", "KB", "MB", "GB", "TB"];

let data = [];

let parent = null;

function format_size(input) {
  let current = 0;
  input = Number(input);

  while (input > 1024) {
    input /= 1024;
    current++;
  }
  return input.toFixed(2) + " " + sizes[current];
}

async function main() {
  await listen("tauri://close-requested", async () => {
    localStorage.clear();
  });

  await listen("task-update", (event) => {
    let json = JSON.parse(event.payload);

    let found = false;
    for (let index = 0; index < data.length; index++) {
      const element = data[index];
      if (element.id == json.id) {
        data[index] = json;
        found = true;
      }
    }

    if (!found) {
      data.push(json);
    }
    data.sort((a, b) => b.size - a.size);

    localStorage.setItem("tasks", JSON.stringify(data));
    updateUi();
  });
}
function updateUi() {
  parent.innerHTML = "";

  for (let index = 0; index < data.length; index++) {
    const element = data[index];
    let id = element.id;

    const child = document.createElement("div");
    child.id = id;
    child.className = "list-item";

    let html = element_html
      .replaceAll("[id]", id)
      .replace("[name]", element.name)
      .replace("[downloaded]", format_size(element.downloaded))
      .replace("[size]", format_size(element.size))
      .replace("[speed]", format_size(element.speed) + "/s")
      .replace("[progress]", (element.downloaded / element.size) * 100);

    child.innerHTML = html;

    parent.appendChild(child);
  }
}
document.addEventListener("DOMContentLoaded", async () => {
  let data_storage = localStorage.getItem("tasks");
  if (data_storage != null) {
    data = JSON.parse(data_storage);
  }

  parent = document.getElementById("download_file");
  updateUi();

  document.getElementById("clear_downloads").addEventListener("click", () => {
    data = [];
    parent.innerHTML = "";
    localStorage.removeItem("tasks");
  });

  main();
});
