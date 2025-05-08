mod utils;
mod commands;

use std::{ fs, os::unix::thread, path::Path, sync::Mutex };
use tauri::{ Emitter, Manager };

use threadpool::ThreadPool;

use utils::{ error::BackupError, settings::Settings, structs::{ BackupInfo, BackupItem }, * };
use commands::backups;

struct AppData {
    running: Mutex<bool>,
    threadpool: ThreadPool,
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .setup(|app| {
            app.manage(AppData {
                running: Mutex::from(false),
                threadpool: ThreadPool::new(6),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                backups::backup,
                backups::add_backup,
                backups::remove_backup,
                backups::get_backups
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
