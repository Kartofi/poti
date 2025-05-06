import { add_backup, get_backups, backup } from "/js/tauri-commands.js";
import { backup_el } from "/js/data.js";
import { clear_tasks } from "/js/tasks.js";
let parent = null;

let backups_old = [];
let times_old = [];

window.addEventListener("DOMContentLoaded", async () => {
  parent = document.getElementById("backups");

  times_old = JSON.parse(localStorage.getItem("backups_times")) || [];

  document
    .getElementById("reload_backups")
    .addEventListener("click", async () => {
      await updateUi();
    });

  await updateUi();
  document.getElementById("backup").addEventListener("click", async (e) => {
    e.preventDefault();
    let id = e.target.getAttribute("backup_id");

    try {
      clear_tasks();
      await backup(id);

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
      document.getElementById(id + "-time").innerText = time.toISOString();
      localStorage.setItem("backups_times", JSON.stringify(times_old));
    } catch (e) {}
  });
});

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
        document.getElementById(id + "-time").innerText = found.time;
      }
    }
  }
}
