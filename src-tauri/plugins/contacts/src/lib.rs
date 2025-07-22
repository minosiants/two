#![cfg(mobile)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Contacts;
#[cfg(mobile)]
use mobile::Contacts;

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
<<<<<<< Updated upstream
  Builder::new("contacts")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let contacts = mobile::init(app, api)?;
      #[cfg(desktop)]
      let contacts = desktop::init(app, api)?;
      app.manage(contacts);
      Ok(())
    })
    .build()
=======
    Builder::new("contacts")
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            let contacts = mobile::init(app, api)?;
            app.manage(contacts);
            Ok(())
        })
        .build()
>>>>>>> Stashed changes
}
