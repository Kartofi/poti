use std::{ fs, os::unix::thread, path::Path, sync::Mutex };
use reqwest::{ blocking::Client, StatusCode };
use tauri::{ Emitter, Manager };

use threadpool::ThreadPool;
use crate::utils::{
    error::BackupError,
    settings::Settings,
    structs::{ BackupInfo, BackupItem },
    *,
};

use crate::AppData;

#[tauri::command]
pub fn backup(window: tauri::Window, id: String) -> Result<(), BackupError> {
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

    let client = Client::new();

    let resp = client
        .get(backup.url.to_owned() + "/journal")
        .header("secret", &backup.secret)
        .send();
    if resp.is_err() {
        return Err(BackupError::new(true, "Can't connect to the server!"));
    }
    let resp = resp.unwrap();
    if resp.status() != StatusCode::OK {
        return Err(BackupError::new(true, "Wrong secret!"));
    }

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

    backupitem_server.sync(window, &data.threadpool, &backup.secret, &backup.id, &backup.url)
}

#[tauri::command]
pub fn add_backup(backup_info: BackupInfo) -> Result<BackupInfo, BackupError> {
    let mut settings: Settings = Settings::new()?;

    let found = settings.backups
        .iter()
        .find(|item| (item.path == backup_info.path || item.url == backup_info.url));

    if found.is_some() {
        return Err(BackupError::new(true, "Backup with the same url or path exists!"));
    }
    let mut backup_info = backup_info;
    backup_info.gen_id();
    // Check secret/url
    let client = Client::new();

    let resp = client
        .get(backup_info.url.to_owned() + "/journal")
        .header("secret", &backup_info.secret)
        .send();
    if resp.is_err() || resp.unwrap().status() != StatusCode::OK {
        return Err(BackupError::new(true, "Invalid server or invalid secret!"));
    }
    //

    settings.backups.push(backup_info.clone());

    settings.save()?;

    Ok(backup_info)
}
#[tauri::command]
pub fn remove_backup(window: tauri::Window, id: String) -> Result<(), BackupError> {
    let mut settings: Settings = Settings::new()?;

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
        return Err(BackupError::new(true, "Backup is running!"));
    }

    let found = settings.backups.iter().position(|item| item.id == id);

    if found.is_none() {
        return Err(BackupError::new(true, "No backup found with that id!"));
    }
    let backup = &settings.backups[found.clone().unwrap()];

    if Path::new(&backup.path).exists() {
        match fs::remove_dir_all(&backup.path) {
            Ok(()) => {}
            Err(e) => {
                println!("{:?}", backup.path);
                return Err(BackupError::new(true, "Can't delete directory!"));
            }
        }
    }

    settings.backups.remove(found.unwrap());

    settings.save()?;

    Ok(())
}
#[tauri::command]
pub fn get_backups() -> Result<Vec<BackupInfo>, BackupError> {
    let settings: Settings = Settings::new()?;
    let mut backups = settings.backups;

    for backup in &mut backups {
        backup.update_size();
    }

    Ok(backups)
}
