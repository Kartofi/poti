const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

async function backup(id) {
  let res = await invoke("backup", { id: id });
  console.log("done");
  return res;
}
async function get_backups() {
  let res = await invoke("get_backups");

  return res;
}
async function add_backup() {
  let res = await invoke("add_backup", {
    backupInfo: {
      name: "Hi",
      id: "",
      url: "http://localhost:3003",
      path: "/home/anatoli/Desktop/Poti/poti-client/backup3",
    },
  });

  window.alert(JSON.stringify(res));
}

let element_html = `<a id="[id]-name">[name]</a><br>
        <a id="[id]-path">[path]</a><br>
        <a id="[id]-url">[url]</a><br>
        <button id="backup" backup_id="[id]">Back up NOW</button>
      `;
let parent = null;

let backups_old = [];

window.addEventListener("DOMContentLoaded", async () => {
  // main();

  parent = document.getElementById("backups");
  document
    .getElementById("reload_backups")
    .addEventListener("click", async () => {
      await updateUi();
    });

  await updateUi();
  document.getElementById("backup").addEventListener("click", (e) => {
    e.preventDefault();
    backup(e.target.getAttribute("backup_id"));
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

      let html = element_html
        .replaceAll("[id]", id)
        .replace("[name]", element.name)
        .replace("[path]", element.path)
        .replace("[url]", element.url);
      child.innerHTML = html;

      parent.appendChild(child);
      continue;
    }
  }
}
