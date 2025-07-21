use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_contacts);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Contacts<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.minosiants.two.contacts", "ContactsPlugin")?;
    // #[cfg(target_os = "ios")]
    // let handle = api.register_ios_plugin(init_plugin_contacts)?;
    Ok(Contacts(handle))
}

/// Access to the contacts APIs.
pub struct Contacts<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Contacts<R> {
    pub fn contacts(&self) -> crate::Result<Vec<Contact>> {
        self.0.run_mobile_plugin("contacts", ()).map_err(Into::into)
    }
}
