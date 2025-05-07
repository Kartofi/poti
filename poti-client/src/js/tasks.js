const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const appWindow = window.__TAURI__.window;

import { format_size } from "/js/utils.js";
import { task_el } from "/js/data.js";

let data = [];

let parent = null;

function clear_tasks() {
  data = [];
  parent.innerHTML = "";
  localStorage.removeItem("tasks");
}

async function listen_events() {
  await listen("tauri://close-requested", async () => {
    localStorage.removeItem("tasks");
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

    let html = task_el
      .replaceAll("[id]", id)
      .replace("[name]", element.name)
      .replace("[downloaded]", format_size(element.downloaded))
      .replace("[size]", format_size(element.size))
      .replace("[speed]", format_size(element.speed) + "/s")
      .replaceAll(
        "[progress]",
        ((element.downloaded / element.size) * 100).toFixed(2)
      );

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
    clear_tasks();
  });

  listen_events();
});

export { clear_tasks };
