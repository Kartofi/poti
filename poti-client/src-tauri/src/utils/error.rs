use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupError {
    pub is_critical: bool,
    pub message: String,
}
impl BackupError {
    pub fn new(is_critical: bool, message: &str) -> BackupError {
        BackupError { is_critical: is_critical, message: message.to_string() }
    }
}
