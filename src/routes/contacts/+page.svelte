<script>
  import Contacts from "$lib/components/Contacts.svelte";
  import Search from "$lib/components/Search.svelte";
  import { setContext } from "svelte";
  import _contacts from "$lib/js/contacts";
  let contacts = $state(_contacts);
  setContext("contacts", contacts);
</script>

<main class="stack box">
  <Search />
  <Contacts {contacts} />
</main>

<style>
</style>
