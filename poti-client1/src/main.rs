use std::{ collections::HashMap, fs, path::Path };

use structs::BackupItem;
use utils::format_size;
mod structs;
mod downloader;
mod error;
mod utils;

pub static URL: &str = "http://localhost:3000";
fn main() {
    let resp = reqwest::blocking::get(URL.to_string() + "/journal").unwrap();

    if Path::new("./backup").exists() {
        fs::remove_dir_all("./backup").unwrap();
    }

    let mut backupitem_server: BackupItem = resp.json::<BackupItem>().unwrap();

    backupitem_server.sync();
    loop {
    }
}
