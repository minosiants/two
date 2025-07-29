<script>
  import Contacts from "$lib/components/Contacts.svelte";
  import Search from "$lib/components/Search.svelte";
  import { setContext } from "svelte";
  import c from "$lib/js/contacts";
  let contacts = $state(c.contacts);
  setContext("contacts", contacts);
</script>

<main class="stack box">
  <Search />
  <Contacts {contacts} />
</main>

<style>
</style>
