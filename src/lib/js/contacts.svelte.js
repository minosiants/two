import { listen, TauriEvent } from "@tauri-apps/api/event";
import {
  getContacts as queryContacts,
  checkPermissions,
  requestPermissions,
} from "tauri-plugin-contacts";
import {
  exists,
  readTextFile,
  mkdir,
  create,
  remove,
  BaseDirectory,
} from "@tauri-apps/plugin-fs";

const appDir = "data";
const contactsFile = `${appDir}/two.json`;
const options = { baseDir: BaseDirectory.AppConfig, recursive: true };

async function readContacts() {
  const fileExists = await exists(contactsFile, options);
  if (!fileExists) return [];

  return readTextFile(contactsFile, options).then(fromJson);
}

const fromJson = async (str) => {
  try {
    return JSON.parse(str);
  } catch (e) {
    return [];
  }
};

const log = (msg) => async (d) => {
  console.log(`msg ${msg} d: ${d}`);
  return d;
};

//////////////////////

export let contactsState = $state({ value: [] });

export const deleteFile = async () => remove(contactsFile, options);

export const reset = async () => {
  await deleteFile();
  const contacts = await getContacts();
  contactsState.value.splice(0, contactsState.value.length, ...contacts);
};

export async function saveContacts(contacts) {
  const fileExits = await exists(contactsFile, options);

  const removeFile = async () => await remove(contactsFile, options);

  const saveFile = async () => {
    const file = await create(contactsFile, options);
    const data = new TextEncoder().encode(JSON.stringify(contacts));
    await file.write(data);
    await file.close();
  };

  if (fileExits) return removeFile().then(saveFile);

  return saveFile().catch(log("faild to delete"));
}

const fromUserContacts = async () => {
  let state = await checkPermissions();
  if (state.readContacts === "prompt") state = await requestPermissions();
  if (state.readContacts !== "granted") {
    console.error("Dont have access to contacts");
    return [];
  } else {
    const contacts = await queryContacts();
    return contacts.map((contact) => {
      return { ...contact, ...{ selected: false, success: 0, fail: 0 } };
    });
  }
};

export const getContacts = async () =>
  readContacts()
    .then(async (c) => {
      console.log("contacts result");
      console.log(c);
      if (c.length === 0) return await fromUserContacts();
      else {
        return c;
      }
    })
    .catch((err) => {
      console.error("Error in getContacts:", err);
    });
export const ensureBaseDir = async () => {
  const options = { baseDir: BaseDirectory.AppData, recursive: true };
  var mk = async (isThere) =>
    isThere ? Promise.resolve() : mkdir(appDir, options);
  exists(appDir, options).then(mk).catch(log);
};

export const init = async () => {
  await ensureBaseDir();
  listen(TauriEvent.WINDOW_DESTROYED, async () => {
    saveContacts(contactsState.value);
  });
};
