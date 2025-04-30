use std::{ fs::File, io::Read };

pub struct Settings {
    pub backup_path: String,
}
impl Settings {
    pub fn new(backup_path: String) -> Settings {
        Settings { backup_path: backup_path }
    }
    pub fn default() -> Settings {
        Self::new(String::new())
    }
    pub fn load_path(settings_path: String) -> Settings {
        let mut settings = Settings::new(settings_path);
        settings.load();
        return settings;
    }
    pub fn load(&mut self) {
        let mut file = File::open("./settings.poti").unwrap();

        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let lines: Vec<&str> = data.lines().collect();

        self.backup_path = lines[0]["backup_path=".len()..].to_string();
    }
}
