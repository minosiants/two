<script>
  import Contacts from "$lib/components/Contacts.svelte";
  import Search from "$lib/components/Search.svelte";
  import { contactsState, reset } from "$lib/js/contacts.svelte";
  let contacts = contactsState.value;
  const onclick = async () => {
    await reset();
    console.log("snapshot");
    console.log($state.snapshot(contactsState));
  };
  $effect(() => {
    console.log("!!! update");
    console.log($state.snapshot(contactsState));
  });
</script>

<main class="stack box">
  {#if contacts.length !== 0}
    <Search bind:contacts />
    <Contacts bind:contacts />
  {/if}
  <button {onclick}> reset </button>
</main>

<style>
</style>
