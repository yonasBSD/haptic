// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
mod mac;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(target_os = "macos") {
          #[cfg(target_os = "macos")]
          use mac::window::setup_mac_window;

          #[cfg(target_os = "macos")]
          setup_mac_window(app);
      }

        Ok(())
    })
    .plugin(tauri_plugin_fs_watch::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
