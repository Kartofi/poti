use std::{ fs::{ File, OpenOptions }, io::{ Read, Write } };

use super::id;

pub struct Settings {
    pub backup_path: String,
    pub secret: String,
}
impl Settings {
    pub fn new(backup_path: String, secret: String) -> Settings {
        Settings { backup_path: backup_path, secret: secret }
    }
    pub fn default() -> Settings {
        Self::new(String::new(), String::new())
    }
    pub fn load_path(settings_path: String) -> Settings {
        let mut settings = Settings::new(settings_path, String::new());
        settings.load();
        return settings;
    }
    pub fn load(&mut self) {
        let mut file = File::open("./settings.poti").unwrap();

        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let lines: Vec<&str> = data.lines().collect();

        self.backup_path = lines[0]["backup_path=".len()..].to_string();
        if lines.len() == 2 {
            self.secret = lines[1]["secret=".len()..].to_string();
            return;
        }
        let mut file = OpenOptions::new().append(true).open("./settings.poti").unwrap();

        let secret = id::gen_id();
        file.write_all(format!("secret={}", &secret).as_bytes()).unwrap();
        self.secret = secret;
    }
}
