use lazy_static::lazy_static;
use choki::*;

mod utils;
use utils::{ files::BackupItem, settings::* };

lazy_static! {
    static ref SETTINGS: Settings = Settings::load_path("./settings.poti".to_string());
}
fn main() {
    let mut backup_items = BackupItem::new(false, SETTINGS.backup_path.clone());

    let mut server: Server<BackupItem> = Server::new(Some(0), Some(backup_items));

    server
        .get("/journal", |req, mut res, public_var| {
            let mut backup = public_var.unwrap();
            backup.scaffold_initial().unwrap();

            res.send_json(&serde_json::to_string(&backup).unwrap())
        })
        .unwrap();

    server.new_static("/backup", &SETTINGS.backup_path).unwrap();

    server.listen(3000, None, Some(6), || { println!("Listening on port 3000!") }).unwrap();

    Server::<u8>::lock();
}
