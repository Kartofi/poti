use std::path::Path;

use lazy_static::lazy_static;
use choki::*;

mod utils;
use utils::{ files::BackupItem, settings::* };

static CONFIG: &str = "./settings.poti";

lazy_static! {
    static ref SETTINGS: Settings = Settings::load_path();
}
fn main() {
    let backup_items = BackupItem::new(false, SETTINGS.backup_path.to_string(), true);

    let mut server: Server<BackupItem> = Server::new(Some(0), Some(backup_items));

    server.use_middleware(|url, req, res, public_var| {
        let headers = &req.headers;

        let found = headers.iter().find(|item| item.name == "secret");
        if found.is_none() {
            res.set_status(&src::structs::ResponseCode::BadRequest);
            res.send_string("No secret provided!").unwrap();
            return false;
        }
        let secret = found.unwrap();
        if secret.value != SETTINGS.secret {
            res.set_status(&src::structs::ResponseCode::BadRequest);
            res.send_string("Wrong secret!").unwrap();
            return false;
        }
        return true;
    });

    server
        .get("/journal", |req, mut res, public_var| {
            let mut backup = public_var.unwrap();
            backup.scaffold_initial().unwrap();

            res.send_json(&serde_json::to_string(&backup).unwrap())
        })
        .unwrap();

    server.new_static("/backup", &SETTINGS.backup_path).unwrap();

    server.listen(3000, None, Some(6), || { println!("Api is listening on port 3000!") }).unwrap();

    Server::<u8>::lock();
}
