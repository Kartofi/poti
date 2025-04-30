const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

async function backup() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  let res = await invoke("backup", { id: "123" });

  window.alert(JSON.stringify(res));
}

async function add_backup() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  let res = await invoke("add_backup", {
    backupInfo: {
      name: "Hi",
      id: "",
      url: "http://localhost:30001",
      path: "/home/anatoli/Desktop/Poti/poti-client/backup1",
    },
  });

  window.alert(JSON.stringify(res));
}
async function main() {
  await listen("frontend-event", (event) => {
    console.log("Received event:", event.payload);
  });
}
main();
window.addEventListener("DOMContentLoaded", () => {
  main();

  document.getElementById("backup").addEventListener("click", (e) => {
    e.preventDefault();
    add_backup();
  });
});
