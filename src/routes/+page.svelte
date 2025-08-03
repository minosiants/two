<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import { contactsStore } from "$lib/js/store";
  import { onDestroy } from "svelte";

  let contacts = $state([]);
  let unsubscirbe = contactsStore.subscribe((v) => (contacts = v));
  onDestroy(unsubscirbe);
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
