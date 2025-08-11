use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::ContactsExt;
use crate::Result;

#[command]
pub(crate) async fn check_permissions<R: Runtime>(app: AppHandle<R>) -> Result<PermissionStatus> {
    app.contacts().check_permissions()
}
#[command]
pub(crate) async fn request_permissions<R: Runtime>(
    app: AppHandle<R>,
    // permissions: RequestPermissionsArgs,
) -> Result<PermissionStatus> {
    app.contacts().request_permissions()
}
#[command]
pub(crate) async fn get_contacts<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Contact>> {
    app.contacts().get_contacts()
}
