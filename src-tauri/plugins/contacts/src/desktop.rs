use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::error::*;
use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Contacts<R>> {
    Ok(Contacts(app.clone()))
}

/// Access to the contacts APIs.
pub struct Contacts<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Contacts<R> {
    pub fn check_permissions(&self) -> Result<PermissionStatus> {
        Err(Error::PlatformNotSupported)
    }
    pub fn request_permissions(&self) -> Result<PermissionStatus> {
        Err(Error::PlatformNotSupported)
    }
    pub fn get_contacts(&self) -> Result<Vec<Contact>> {
        println!("!!!!!!!!!!!! get_contact not implemented");
        Err(Error::PlatformNotSupported)
    }
}
