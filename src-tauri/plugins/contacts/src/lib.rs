#![cfg(mobile)]

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

mod error;
mod mobile;
mod models;

pub use error::{Error, Result};

pub use mobile::Contacts;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the contacts APIs.
pub trait ContactsExt<R: Runtime> {
    fn contacts(&self) -> &Contacts<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ContactsExt<R> for T {
    fn contacts(&self) -> &Contacts<R> {
        self.state::<Contacts<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("contacts")
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            let contacts = mobile::init(app, api)?;
            app.manage(contacts);
            Ok(())
        })
        .build()
}
