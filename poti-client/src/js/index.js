const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

async function backup() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  let res = await invoke("backup", { id: "16967331108002224150" });

  console.log(JSON.stringify(res));
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

window.addEventListener("DOMContentLoaded", () => {
  main();

  document.getElementById("backup").addEventListener("click", (e) => {
    e.preventDefault();
    add_backup();
  });
});
