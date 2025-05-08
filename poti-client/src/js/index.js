const { listen } = window.__TAURI__.event;

import {
  add_backup,
  remove_backup,
  get_backups,
  backup,
} from "/js/tauri-commands.js";

import { backup_el } from "/js/data.js";
import { format_time } from "/js/utils.js";

import { clear_tasks } from "/js/tasks.js";
let parent = null;

let backups_old = [];
let times_old = [];

function update_backup_time(id) {
  let time = new Date();

  let found = false;

  for (let index = 0; index < times_old.length; index++) {
    const element = times_old[index];
    if (element.id == id) {
      element.time = time;
      found = true;
      break;
    }
  }
  if (found == false) {
    times_old.push({ id: id, time: time });
  }
  document.getElementById(id + "-time").innerText = format_time(
    time.toISOString()
  );
  localStorage.setItem("backups_times", JSON.stringify(times_old));
}

async function updateUi() {
  let backups = await get_backups();
  if (backups_old == backups) {
    return;
  }

  parent.innerHTML = "";
  for (let index = 0; index < backups.length; index++) {
    const element = backups[index];
    let id = element.id;

    let el = document.getElementById(id);

    if (el == null) {
      const child = document.createElement("div");
      child.id = id;
      child.className = "list-item";

      let html = backup_el
        .replaceAll("[id]", id)
        .replace("[name]", element.name)
        .replace("[path]", element.path)
        .replace("[url]", element.url);
      child.innerHTML = html;

      let found = null;

      for (let index = 0; index < times_old.length; index++) {
        const element = times_old[index];
        if (element.id == id) {
          found = element;
          break;
        }
      }

      parent.appendChild(child);

      if (found != null) {
        document.getElementById(id + "-time").innerText = format_time(
          found.time
        );
      }
    }
  }
}
async function listen_events() {
  await listen("backup-done", (event) => {
    let id = event.payload;

    update_backup_time(id);
  });

  await listen("backup-error", (event) => {
    let json = JSON.parse(event.payload);
    console.log(json);
  });
}
window.addEventListener("DOMContentLoaded", async () => {
  await listen_events();

  parent = document.getElementById("backups");

  try {
    times_old = JSON.parse(localStorage.getItem("backups_times")) || [];
  } catch (e) {
    times_old = [];
  }

  document
    .getElementById("reload_backups")
    .addEventListener("click", async () => {
      await updateUi();
    });

  await updateUi();

  let backup_buttons = document.querySelectorAll("#backup");

  backup_buttons.forEach((element) => {
    element.addEventListener("click", async (e) => {
      e.preventDefault();

      clear_tasks();
      await backup(e.target.getAttribute("backup_id"));
    });
  });

  let remove_backup_buttons = document.querySelectorAll("#remove_backup");

  remove_backup_buttons.forEach((element) => {
    element.addEventListener("click", async (e) => {
      e.preventDefault();
      try {
        let id = e.target.getAttribute("backup_id");
        await remove_backup(id);

        for (let index = 0; index < times_old.length; index++) {
          const element = times_old[index];
          if (element.id == id) {
            found = true;
            break;
          }
        }
        if (found == true) {
          times_old.splice(index, 1);
          localStorage.setItem("backups_times", JSON.stringify(times_old));
        }
      } catch (e) {
        console.log(e);
      }
    });
  });
});
