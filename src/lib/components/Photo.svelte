<script>
  import Avatar from "./svg/Avatar.svelte";

  const { photo = "" } = $props();
</script>

{#if photo != ""}
  <img src={photo} alt="" />
{:else}
  <Avatar />
{/if}
