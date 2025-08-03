<script>
  import Plus from "./svg/Plus.svelte";
  import Minus from "./svg/Minus.svelte";
  let { toggeled = $bindable() } = $props();
  const onclick = () => {
    toggeled = !toggeled;
  };
</script>

<button {onclick}>
  {#if toggeled}
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
