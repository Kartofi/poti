use std::{ fs::{ self, OpenOptions }, io::{ Read, Write }, thread };

use reqwest::blocking::Client;
use serde::{ Deserialize, Serialize };
use urlencoding::encode;

use crate::URL;
use crate::downloader;

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
    pub fn sync(&mut self) {
        if self.is_file == true {
            let url = URL.to_string() + &self.url.replace(&self.name, &encode(&self.name));
            let path = ".".to_string() + &self.url;
            thread::spawn(move || {
                downloader::download(&url, &path);
            });
            return;
        }
        fs::create_dir(".".to_string() + &self.url).unwrap_or_default();
        for child in &mut self.children {
            child.sync();
        }
    }
}
