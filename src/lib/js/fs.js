import { exists, readTextFile, writeTextFile, remove, BaseDirectory } from '@tauri-apps/plugin-fs';

const contactsFile = "two.json";
const options = { baseDir: BaseDirectory.AppLocalData }

export async function readContacts() {
  const fileExists = await exists(contactsFile, options);
  if (!fileExists)
    return [];

  const data = await readTextFile(contactsFile, options);
  return JSON.parse(data);
}

export async function saveContacts(contacts) {

  const fileExits = await exists(contactsFile, options);
  const removeFile = async () => await remove(contactsFile, options);
  const saveFile = async () => await writeTextFile(contactsFile, JSON.stringify(contacts), options);
  if (fileExits)
    return removeFile().then(saveFile);
  return saveFile();

}
