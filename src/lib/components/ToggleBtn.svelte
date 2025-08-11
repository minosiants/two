<script>
  import Plus from "./svg/Plus.svelte";
  import Minus from "./svg/Minus.svelte";
  import { contactsState } from "$lib/js/contacts.svelte";
  let { toggled = $bindable() } = $props();
  const onclick = () => {
    console.log($state.snapshot(contactsState.value));
    toggled = !toggled;
    console.log($state.snapshot(contactsState.value));
  };
</script>

<button {onclick}>
  {#if toggled}
    <Minus />
  {:else}
    <Plus />
  {/if}
</button>

<style>
  button {
    width: 2rem;
  }
</style>
