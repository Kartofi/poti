use std::{ fs::{ File, OpenOptions }, io::{ Read, Write } };

use crate::CONFIG;

use super::id;

pub struct Settings {
    pub secret: String,
    pub backup_path: String,
}
impl Settings {
    pub fn new(secret: String, backup_path: String) -> Settings {
        Settings { secret: secret, backup_path: backup_path }
    }
    pub fn default() -> Settings {
        Self::new(String::new(), String::new())
    }
    pub fn load_path() -> Settings {
        let mut settings = Settings::new(String::new(), String::new());
        settings.load();
        return settings;
    }
    pub fn load(&mut self) {
        println!("Loading settings...");

        let mut file = File::open(CONFIG).unwrap();

        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let lines: Vec<&str> = data.lines().collect();

        println!("Done loading settings path...");

        self.backup_path = lines[0]["backup_path=".len()..].to_string();

        if lines.len() > 1 {
            self.secret = lines[1]["secret=".len()..].to_string();
            println!("Your secret is {}", self.secret);
            return;
        }
        let mut file = OpenOptions::new().append(true).open(CONFIG).unwrap();

        let secret = id::gen_id();
        file.write_all(format!("\nsecret={}", &secret).as_bytes()).unwrap();
        self.secret = secret;
        println!("Your secret is {}", self.secret);
    }
}
