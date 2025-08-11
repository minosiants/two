// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod model;

#[cfg(mobile)]
use std::error::Error;
use std::{print, vec};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .plugin(
        //     tauri_plugin_log::Builder::new()
        //         .target(tauri_plugin_log::Target::new(
        //             tauri_plugin_log::TargetKind::Stdout,
        //         ))
        //         .build(),
        // )
        .plugin(contacts::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        //.invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
