// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
<<<<<<< Updated upstream
#[tauri::command]
fn greet(name: &str) -> String {
=======
//#![cfg(mobile)]
use contacts::{
    self, //    , Contacts
          //  , ContactsExt
};

#[tauri::command]
fn greet(app: tauri::AppHandle, name: &str) -> String {
    //let result = app.contacts().contacts();
    //println!("result {:?}", result);
>>>>>>> Stashed changes
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
<<<<<<< Updated upstream
=======
        //  .plugin(contacts::init())
>>>>>>> Stashed changes
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
