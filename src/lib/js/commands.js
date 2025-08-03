import { invoke } from "@tauri-apps/api/core";


export async function getContacts() {
  return await invoke("get_contacts")
    .then((message) => {
      console.log(message.length);
      return message;
    }
    );
}
export async function saveContacts(contacts) {
  return await invoke("save_contacts", { contacts: contacts });
}
