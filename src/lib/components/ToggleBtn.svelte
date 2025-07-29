<script>
  import Plus from "./svg/Plus.svelte";
  import Minus from "./svg/Minus.svelte";
  import { getContext } from "svelte";
  const { id } = $props();
  let contact = getContext("contacts").find((v) => v.id === id);
  const onclick = () => {
    contact.selected = !contact.selected;
  };
</script>

<button {onclick}>
  {#if contact.selected}
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
