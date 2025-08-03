// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![cfg(mobile)]

mod model;

#[cfg(mobile)]
use contacts::{self, Contact, Contacts, ContactsExt, Result};
use model::Contact as Cont;
use std::error::Error;
use std::{print, vec};

#[cfg(mobile)]
#[tauri::command]
fn get_contacts(app: tauri::AppHandle) -> Vec<Cont> {
    let res: Vec<Contact> = app.contacts().contacts().unwrap_or(vec![]);
    let result = res
        .iter()
        .map(|mut v: &Contact| Cont::new(&v.id, &v.name, &v.phone))
        .collect();
    println!("result {:?}", result);
    result
}
#[tauri::command]
fn save_contacts(app: tauri::AppHandle, contacts: Vec<Cont>) -> bool {
    println!("save contacts {:?}", contacts);
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(contacts::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_contacts, save_contacts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
