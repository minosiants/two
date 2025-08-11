import { invoke } from "@tauri-apps/api/core";

export type PermissionStatus = {
  readContacts: string;
};

export interface Contact {
  id: string;
  name: string;
  phone: string;
}
export type RequestPermissionsArgs = {
  permissions: string[];
};

export async function checkPermissions(): Promise<PermissionStatus> {
  return await invoke("plugin:contacts|check_permissions");
}
export async function requestPermissions(): Promise<PermissionStatus> {
  return await invoke("plugin:contacts|request_permissions");
}
export async function getContacts(): Promise<Contact[]> {
  return await invoke("plugin:contacts|get_contacts");
}
