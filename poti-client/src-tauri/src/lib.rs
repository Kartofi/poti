mod utils;
use std::{ fs, os::unix::thread, path::Path, sync::Mutex };
use tauri::{ Emitter, Manager };

use threadpool::ThreadPool;
use utils::{ error::BackupError, settings::Settings, structs::{ BackupInfo, BackupItem }, * };

pub static URL: &str = "http://localhost:3000";

#[tauri::command]
fn backup(window: tauri::Window, id: String) -> Result<(), BackupError> {
    let window_clone = window.clone();
    let data = window_clone.state::<AppData>();

    let mut running = data.running.lock().unwrap();
    if
        *running == true &&
        data.threadpool.queued_count() == 0 &&
        data.threadpool.active_count() == 0
    {
        *running = false;
    }
    if *running == true {
        return Err(BackupError::new(true, "Backup is already running!"));
    }

    let mut settings = Settings::new()?;

    let backup = settings.backups.iter().find(|item| item.id == id);
    if backup.is_none() {
        return Err(BackupError::new(true, "Invalid backup id!"));
    }
    let backup = backup.unwrap();

    let resp = reqwest::blocking::get(backup.url.to_owned() + "/journal").unwrap();

    let mut backupitem_server: BackupItem = resp.json::<BackupItem>().unwrap();

    fn iter_children_path(backupitem: &mut BackupItem, prefix: String) {
        backupitem.path = prefix.clone() + &backupitem.path;
        if backupitem.is_file {
            return;
        }
        for child in &mut backupitem.children {
            iter_children_path(child, prefix.clone());
        }
    }
    iter_children_path(&mut backupitem_server, backup.path.to_owned());

    if Path::new(&backupitem_server.path).exists() {
        fs::remove_dir_all(&backupitem_server.path).unwrap();
    }

    *running = true;

    backupitem_server.sync(window, &data.threadpool);

    Ok(())
}

#[tauri::command]
async fn add_backup(backup_info: BackupInfo) -> Result<BackupInfo, BackupError> {
    let mut settings: Settings = Settings::new()?;

    let found = settings.backups
        .iter()
        .find(|item| (item.path == backup_info.path || item.url == backup_info.url));

    if found.is_some() {
        return Err(BackupError::new(true, "Backup with the same url or path exists!"));
    }
    let mut backup_info = backup_info;
    backup_info.gen_id();

    settings.backups.push(backup_info.clone());

    settings.save()?;

    Ok(backup_info)
}
#[tauri::command]
async fn remove_backup(id: String) -> Result<(), BackupError> {
    let mut settings: Settings = Settings::new()?;

    let found = settings.backups.iter().position(|item| item.id == id);

    if found.is_none() {
        return Err(BackupError::new(true, "No backup found with that id!"));
    }

    settings.backups.remove(found.unwrap());

    settings.save()?;

    Ok(())
}
#[tauri::command]
async fn get_backups() -> Result<Vec<BackupInfo>, BackupError> {
    let settings: Settings = Settings::new()?;

    Ok(settings.backups)
}
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
        .invoke_handler(tauri::generate_handler![backup, add_backup, remove_backup, get_backups])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
