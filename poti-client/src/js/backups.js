const { listen } = window.__TAURI__.event;

import {
  add_backup,
  remove_backup,
  get_backups,
  backup,
} from "/js/tauri-commands.js";

import { backup_el } from "/js/data.js";
import { format_time, format_size } from "/js/utils.js";

import { clear_tasks } from "/js/tasks.js";

import { show_popup } from "/js/components/popup.js";
import { show_dialog } from "/js/components/dialog.js";

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
  backups_old = backups;

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
        .replace("[url]", element.url)
        .replace("[secret]", element.secret)
        .replace("[size]", format_size(element.size));
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
    let info = null;

    for (let index = 0; index < backups_old.length; index++) {
      const element = backups_old[index];
      console.log(element);
      if (element.id == id) {
        info = element;
        break;
      }
    }
    let data = id;
    if (info != null) {
      data = info.name;
    }
    show_popup('Backup "' + data + '" IS DONE!!!!!');
    update_backup_time(id);
  });

  await listen("backup-error", (event) => {
    let json = JSON.parse(event.payload);
    console.log(json);
    show_popup(JSON.stringify(json));
  });
}
function listen_buttons() {
  let backup_buttons = document.querySelectorAll("#backup");

  backup_buttons.forEach((element) => {
    element.addEventListener("click", async (e) => {
      e.preventDefault();

      show_dialog(
        "Are you sure you want to start a backup?",
        async (result) => {
          if (result == false) {
            return;
          }
          clear_tasks();
          try {
            await backup(e.target.getAttribute("backup_id"));
          } catch (e) {
            show_popup(e.message);
          }
        }
      );
    });
  });

  let remove_backup_buttons = document.querySelectorAll("#remove_backup");

  async function delete_backup(e) {
    try {
      let id = e.target.getAttribute("backup_id");
      await remove_backup(id);

      let found = false;

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
      show_popup(e.message);
    }
  }

  remove_backup_buttons.forEach((element) => {
    element.addEventListener("click", async (e) => {
      e.preventDefault();

      await show_dialog(
        "Are you sure you want to DELETE this backup?",
        async (result) => {
          if (result == false) {
            return;
          }
          await delete_backup(e);
        }
      );
    });
  });

  document.querySelectorAll("#click-copy").forEach((element) => {
    element.addEventListener("click", async (event) => {
      event.preventDefault();
      await navigator.clipboard.writeText(element.innerText);
      show_popup("Copied to clipboard!");
    });
  });
}
window.addEventListener("DOMContentLoaded", async () => {
  parent = document.getElementById("backups");

  await listen_events();

  try {
    times_old = JSON.parse(localStorage.getItem("backups_times")) || [];
  } catch (e) {
    times_old = [];
  }

  document
    .getElementById("reload-backups")
    .addEventListener("click", async () => {
      await updateUi();
      listen_buttons();
      show_popup("Reloaded backups!");
    });

  let new_backup_parent = document.getElementById("new-backup-parent");

  let new_backup_name = document.getElementById("backup-name");
  let new_backup_url = document.getElementById("backup-url");
  let new_backup_secret = document.getElementById("backup-secret");
  let new_backup_path = document.getElementById("backup-path");

  document.getElementById("add-backup").addEventListener("click", async () => {
    new_backup_parent.style.display = "inherit";

    show_dialog("Create new backup:", async (result) => {
      new_backup_parent.style.display = "none";

      if (result == false) {
        return;
      }
      try {
        console.log();
        await add_backup(
          new_backup_name.value,
          new_backup_secret.value,
          new_backup_url.value,
          new_backup_path.value
        );
      } catch (e) {
        show_popup(e.message);
      }

      await updateUi();
    });
  });

  await updateUi();
  listen_buttons();
});
