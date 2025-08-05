<script>
  import Plus from "./svg/Plus.svelte";
  import Minus from "./svg/Minus.svelte";
  let { toggled = $bindable() } = $props();
  const onclick = () => {
    toggled = !toggled;
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
