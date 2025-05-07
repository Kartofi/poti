const { invoke } = window.__TAURI__.core;

async function backup(id) {
  let res = await invoke("backup", { id: id });
  console.log("done");
  return res;
}
async function get_backups() {
  let res = await invoke("get_backups");

  return res;
}
async function add_backup(name, secret, url, path) {
  let res = await invoke("add_backup", {
    backupInfo: {
      name: name,
      id: "",
      secret: secret,
      url: url,
      path: path,
    },
  });

  return res;
}
async function remove_backup(id) {
  let res = await invoke("remove_backup", {
    id,
  });

  return res;
}
export { backup, get_backups, add_backup, remove_backup };
