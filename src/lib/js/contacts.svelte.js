import { invoke } from "@tauri-apps/api/core";
import { listen, TauriEvent } from "@tauri-apps/api/event";

import { exists, readTextFile, writeTextFile, create, remove, BaseDirectory } from '@tauri-apps/plugin-fs';

const contactsFile = "two.json";
const options = { baseDir: BaseDirectory.AppLocalData }

async function readContacts() {
  const fileExists = await exists(contactsFile, options);
  if (!fileExists)
    return [];

  return readTextFile(contactsFile, options).then(fromJson);
}


const fromJson = async (str) => {
  try {
    return JSON.parse(str);

  } catch (e) {
    return [];
  }
}
async function queryContacts() {
  return await invoke("get_contacts")
    .then((message) => {
      console.log(message.length);
      return message;
    }
    );
}




const log = (msg) => async (d) => {
  console.log(msg);
  return d;
}


//////////////////////

export let contactsState = $state({ value: [] });


export async function saveContacts(contacts) {

  const fileExits = await exists(contactsFile, options);
  const removeFile = async () => await remove(contactsFile, options);
  const saveFile = async () =>
    await create(contactsFile, options).then((file) => {
      const data = new TextEncoder().encode(JSON.stringify(contacts));
      file.write(data);
      file.close();
    });
  if (fileExits)
    return removeFile().then(log("file removed")).then(saveFile).then(log("file saved"));
  return saveFile();

}

export const getContacts = async () => readContacts()
  .then((c) => {
    if (c.length === 0) return queryContacts();
    else {
      return c;
    }
  });


export const init = async () => {
  listen(TauriEvent.WINDOW_DESTROYED, async () => {
    saveContacts(contactsState.value);
  });

}
