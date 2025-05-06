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

export { backup, get_backups, add_backup };
