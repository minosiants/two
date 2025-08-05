// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)

import { getContacts, init } from "$lib/js/contacts.svelte";

export const prerender = true;
export const ssr = false;
export async function load() {
  const [_init, contacts] = await Promise.all([
    init(),
    getContacts()
  ]);
  return { contacts: contacts };
}
