use std::{ fs::{ File, OpenOptions }, io::{ Read, Write }, time::Instant };

use reqwest::blocking::Client;

use crate::utils::format_size;

pub fn download(url: &str, path: &str) {
    let client = Client::new();

    let mut response = client.get(url).send().unwrap();

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

    let mut file = File::create(path).unwrap();
    let mut downloaded: u64 = 0;
    let mut buffer = [0; 8192]; // 8KB buffer

    let mut start = Instant::now();
    let mut last_download = 0;

    loop {
        let bytes_read = response.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break; // EOF
        }
        file.write_all(&buffer[..bytes_read]).unwrap();
        downloaded += bytes_read as u64;

        if total_size > 0 {
            let percent = ((downloaded as f64) / (total_size as f64)) * 100.0;

            if start.elapsed().as_secs_f64() >= 0.5 {
                let diff = (downloaded - last_download) as f64;

                println!(
                    "Downloaded: {:.1}% [{}/{}] {}/s",
                    percent,
                    format_size(downloaded),
                    format_size(total_size),
                    format_size((diff / start.elapsed().as_secs_f64()) as u64)
                );
                start = Instant::now();
                last_download = downloaded;
            }
        }
    }

    println!("Download completed successfully. {}", url);
}
