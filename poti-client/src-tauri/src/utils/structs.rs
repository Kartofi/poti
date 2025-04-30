use std::{ fs::{ self, OpenOptions }, io::{ Read, Write }, thread };

use rand::{ self, Rng };
use serde::{ Deserialize, Serialize };
use tauri::{ window, Emitter };
use threadpool::ThreadPool;
use urlencoding::encode;

use crate::{ downloader, URL };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupItem {
    pub is_file: bool,

    pub name: String,
    pub url: String,
    pub path: String,

    pub size: u64,

    pub children: Vec<BackupItem>,
}
impl BackupItem {
    pub fn sync(&mut self, window: tauri::Window, threadpool: &ThreadPool) {
        if self.is_file == true {
            let url = URL.to_string() + &self.url.replace(&self.name, &encode(&self.name));
            let path = self.path.clone();

            let window_clone = window.clone();

            threadpool.execute(move || {
                downloader::download(&url, &path, window_clone);
            });

            return;
        }
        fs::create_dir(&self.path).unwrap_or_default();
        for child in &mut self.children {
            child.sync(window.clone(), threadpool);
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct BackupInfo {
    pub name: String,
    pub id: String,
    pub url: String,
    pub path: String,
}
impl BackupInfo {
    pub fn new(name: String, url: String, path: String) -> BackupInfo {
        let mut backup_info = BackupInfo { name: name, id: "".to_string(), url: url, path: path };
        backup_info.gen_id();

        return backup_info;
    }
    pub fn gen_id(&mut self) {
        let mut rng = rand::rng();
        let id: u64 = rng.random();
        self.id = id.to_string();
    }
}
