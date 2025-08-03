<script>
  import Contacts from "$lib/components/Contacts.svelte";
  import Search from "$lib/components/Search.svelte";
  import { onDestroy } from "svelte";
  import { contactsStore } from "$lib/js/store";
  import { saveContacts } from "$lib/js/commands";
  let contacts = $state([]);
  let unsubscirbe = contactsStore.subscribe((v) => {
    contacts = v;
  });
  $contactsStore = contacts;
  $effect(() => {
    saveContacts(contacts);
  });
  onDestroy(unsubscirbe);
</script>

<main class="stack box">
  {#if contacts.length !== 0}
    <Search bind:contacts />
    <Contacts bind:contacts />
  {/if}
</main>

<style>
</style>
