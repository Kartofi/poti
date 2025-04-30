static SIZES: &[&str] = &["bytes", "KB", "MB", "GB", "TB"];

pub fn format_size(size: u64) -> String {
    let mut index = 0;
    let mut size = size as f64;
    while size > 1024.0 {
        size /= 1024.0;
        index += 1;
    }
    return ((size * 100.0).round() / 100.0).to_string() + " " + SIZES[index];
}
