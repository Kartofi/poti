const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

async function backup() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  let res = await invoke("backup", { id: "xmCrnP-c3ExOW-kauWtc-xRDAf7" });

  console.log(JSON.stringify(res));
}
async function get_backups() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  let res = await invoke("get_backups");

  return res;
}
async function add_backup() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
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

let element_html = `<a id="[id]-name">[name]</a>
        <a id="[id]-path">[path]</a>
        <a id="[id]-url">[url]</a>
        <button id="backup" backup_id="[id]">Back up NOW</button>
      `;
let parent = null;

window.addEventListener("DOMContentLoaded", async () => {
  // main();

  parent = document.getElementById("backups");
  document
    .getElementById("reload_backups")
    .addEventListener("click", async () => {
      parent.innerHTML = "";
      await updateUi();
    });

  await updateUi();
  document.getElementById("backup").addEventListener("click", (e) => {
    console.log(e.target);
    return;
    e.preventDefault();
    backup();
  });
});

async function updateUi() {
  let backups = await get_backups();

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
