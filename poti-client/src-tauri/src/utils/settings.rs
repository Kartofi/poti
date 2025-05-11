use std::{ default, fs::{ File, OpenOptions }, io::{ Read, Write }, path::Path };

use serde::{ Deserialize, Serialize };

use super::{ error::BackupError, structs::BackupInfo };

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Settings {
    pub backups: Vec<BackupInfo>,
}
impl Settings {
    pub fn new() -> Result<Settings, BackupError> {
        let mut settings = Settings::default();
        settings.load()?;
        return Ok(settings);
    }
    pub fn default() -> Settings {
        Settings { backups: Vec::new() }
    }

    pub fn load(&mut self) -> Result<(), BackupError> {
        if !Path::new("./settings.poti").exists() {
            Self::default().save()?;
        }

        let mut file = File::open("./settings.poti").unwrap();

        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let settings: Settings = serde_json::from_str(&data).unwrap_or_default();
        self.backups = settings.backups;
        Ok(())
    }
    pub fn save(&mut self) -> Result<(), BackupError> {
        let mut file = OpenOptions::new().write(true).create(true).open("./settings.poti");
        if file.is_err() {
            return Err(BackupError::new(true, "Cant open/read settings file!"));
        }
        let mut file = file.unwrap();

        for backup in &mut self.backups {
            backup.update_size();
        }

        let data = serde_json::to_string_pretty(&self);
        match data {
            Ok(json) => {
                file.write_all(json.as_bytes()).unwrap();
            }
            Err(e) => {
                return Err(BackupError::new(true, "Cant convert to json!"));
            }
        }

        Ok(())
    }
}
