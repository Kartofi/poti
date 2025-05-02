const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

async function main() {
  await listen("task-update", (event) => {
    console.log("Received event:", event.payload);
  });
}
main();
