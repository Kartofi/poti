use std::{ fs::{ File, OpenOptions }, io::{ Read, Write }, time::Instant };

use reqwest::blocking::Client;
use tauri::{ window, Emitter };

use crate::utils::{ format::format_size, structs::Task };

pub fn download(url: &str, path: &str, secret: &str, window: tauri::Window) {
    let client = Client::new();

    let mut response = client.get(url).header("secret", secret).send().unwrap();

    if !response.status().is_success() {
        eprintln!("Failed to download file: {}", response.status());
        return;
    }
    let total_size: u64 = response
        .headers()
        .get("content-length")
        .unwrap()
        .to_str()
        .unwrap_or_default()
        .parse()
        .unwrap_or(0);

    let mut file = File::create(&path).unwrap();
    let mut downloaded: u64 = 0;
    let mut buffer = [0; 8192]; // 8KB buffer

    let mut start = Instant::now();
    let mut last_download = 0;

    let mut task = Task::new(
        path.split("/").last().unwrap_or_default().to_owned(),
        path.to_owned(),
        0,
        total_size,
        0
    );

    loop {
        let bytes_read = response.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break; // EOF
        }
        file.write_all(&buffer[..bytes_read]).unwrap();
        downloaded += bytes_read as u64;

        if total_size > 0 && start.elapsed().as_secs_f64() >= 1.0 {
            let diff = (downloaded - last_download) as f64;

            task.downloaded = downloaded;
            task.speed = (diff / start.elapsed().as_secs_f64()) as u64;
            window.emit("task-update", task.to_json()).unwrap();

            start = Instant::now();
            last_download = downloaded;
        }
    }
    task.is_done = true;
    task.downloaded = total_size;
    task.speed = 0;

    window.emit("task-update", task.to_json()).unwrap();
}
