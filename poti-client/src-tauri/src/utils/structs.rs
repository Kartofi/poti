use std::{ default, fs::{ self, OpenOptions }, io::{ Read, Write }, thread };

use rand::{ self, Rng };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use tauri::{ window, Emitter };
use threadpool::ThreadPool;
use urlencoding::encode;

use crate::downloader;

use super::{ dir_size, error::BackupError, id::gen_id };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupItem {
    pub is_file: bool,

    pub name: String,
    pub url: String,
    pub path: String,

    pub is_root: bool,

    pub size: u64,

    pub children: Vec<BackupItem>,
}
impl BackupItem {
    pub fn sync(
        &mut self,
        window: tauri::Window,
        threadpool: &ThreadPool,
        secret: &str,
        id: &str,
        url: &str
    ) -> Result<(), BackupError> {
        if self.is_file == true {
            let url = url.to_string() + &self.url.replace(&self.name, &encode(&self.name));
            let path = self.path.clone();
            let secret_clone = secret.to_owned();

            let window_clone = window.clone();

            threadpool.execute(move || {
                match downloader::download(&url, &path, &secret_clone, window_clone) {
                    Ok(()) => (),
                    Err(e) => window.emit("backup-error", e).unwrap(),
                }
            });

            return Ok(());
        }
        fs::create_dir(&self.path).unwrap_or_default();

        for child in &mut self.children {
            child.sync(window.clone(), threadpool, secret, id, url)?;
        }

        if self.is_root {
            let threadpool_clone = threadpool.clone();
            let id_clone = id.to_owned();

            thread::spawn(move || {
                threadpool_clone.join();
                window.emit("backup-done", id_clone).unwrap();
            });
        }
        Ok(())
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct BackupInfo {
    pub name: String,
    pub id: String,
    pub secret: String,

    pub url: String,
    pub path: String,
    pub size: u64,
}
impl BackupInfo {
    pub fn new(name: String, secret: String, url: String, path: String) -> BackupInfo {
        let mut backup_info = BackupInfo {
            name: name,
            secret: secret,
            id: "".to_string(),
            url: url,
            path: path,
            size: 0,
        };
        backup_info.gen_id();

        return backup_info;
    }
    pub fn gen_id(&mut self) {
        self.id = gen_id();
    }
    pub fn update_size(&mut self) {
        self.size = dir_size(self.path.clone()).unwrap_or_default();
    }
}
#[derive(Serialize, Deserialize, Default)]
pub struct Task {
    pub id: String,
    pub is_done: bool,

    pub name: String,
    pub path: String,

    pub downloaded: u64,
    pub size: u64,
    pub speed: u64,
}
impl Task {
    pub fn new(name: String, path: String, downloaded: u64, size: u64, speed: u64) -> Task {
        Task {
            id: gen_id(),
            is_done: false,
            name: name,
            path: path,
            downloaded: downloaded,
            size: size,
            speed: speed,
        }
    }
    pub fn to_json(&mut self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
}
