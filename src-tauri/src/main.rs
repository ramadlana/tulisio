#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod markdown;
mod storage;

use commands::{cleanup_unused_assets, open_file, save_attachment, save_image, save_note};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_note,
            save_image,
            save_attachment,
            cleanup_unused_assets,
            open_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
