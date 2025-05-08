use std::{ fs::{ self, File }, os::unix::fs::MetadataExt };

use serde::{ Deserialize, Serialize };

use crate::SETTINGS;

use super::error::BackupError;

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
    pub fn new(is_file: bool, path: String, is_root: bool) -> BackupItem {
        BackupItem {
            is_file: is_file,
            size: 0,
            name: path.split("/").last().unwrap().to_string(),
            url: path.replace(&SETTINGS.backup_path, "/backup"),
            path: path,
            is_root: is_root,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: BackupItem) {
        self.children.push(child);
    }
    pub fn scaffold(&mut self) -> Result<(), BackupError> {
        let path = self.path.clone();
        self.path = self.path.replace(&SETTINGS.backup_path, "");

        if self.is_file == true {
            self.size = File::open(path.clone()).unwrap().metadata().unwrap().size();
            return Ok(());
        }
        let dir_items = fs::read_dir(&path).unwrap();

        for item in dir_items {
            match item {
                Ok(entry) => {
                    let is_file = entry.file_type().unwrap().is_file();
                    self.add_child(
                        BackupItem::new(
                            is_file,
                            entry.path().to_str().unwrap_or_default().to_string(),
                            false
                        )
                    );
                }
                Err(e) => {
                    return Err(BackupError::new(true, &e.to_string()));
                }
            }
        }
        for child in &mut self.children {
            match child.scaffold() {
                Ok(()) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }
        self.size = self.children
            .iter()
            .map(|item| item.size)
            .sum();
        Ok(())
    }
    pub fn scaffold_initial(&mut self) -> Result<(), BackupError> {
        self.children.clear();
        self.scaffold()
    }
}
