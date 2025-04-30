// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



fn main() {
    #[cfg(target_os = "linux")]
    {
        if
            std::path::Path::new("/dev/dri").exists() &&
            std::env::var("WAYLAND_DISPLAY").is_err() &&
            std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "x11"
        {
            unsafe {
                std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
            }
        }
    }
    poti_client_lib::run()
}
