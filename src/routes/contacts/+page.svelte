<script>
  import Contacts from "$lib/components/Contacts.svelte";
  import Search from "$lib/components/Search.svelte";
  import { contactsState } from "$lib/js/contacts.svelte";
  let contacts = contactsState.value;
</script>

<main class="stack box">
  {#if contacts.length !== 0}
    <Search bind:contacts />
    <Contacts bind:contacts />
  {/if}
</main>

<style>
</style>
