<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import { contactsState } from "$lib/js/contacts.svelte";
  let contacts = contactsState.value;
  console.log("/", $state.snapshot(contacts.length));
</script>

<main class="center cover">
  {#if contacts.length === 0}
    <h1>No contacts</h1>
  {:else}
    <Card {contacts} />
  {/if}
</main>

<style>
</style>
