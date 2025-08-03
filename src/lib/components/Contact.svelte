<script>
  import Photo from "./Photo.svelte";
  import ToggleBtn from "./ToggleBtn.svelte";
  const { contact = $bindable() } = $props();
</script>

<article class="cluster">
  <div class="cluster">
    <div class="photo">
      <Photo photo="" />
    </div>
    <ul class="stack">
      <li>{contact.name}</li>
      <li>{contact.contact}</li>
    </ul>
  </div>
  <div class="toggle">
    <ToggleBtn bind:toggeled={contact.selected} />
  </div>
</article>

<style>
  article {
    --icon-width: 2rem;
    color: var(--color-primary);
    --cluster-justify: space-between;
    --stack-space: var(--s-3);
  }
  ul :first-child {
    font-weight: bold;
  }
  .photo {
    --svg-fill: var(--bgColor-second);
    --svg-stroke: var(--color-primary);
    background-color: var(--ink);
    padding: var(--s-3);
    width: var(--icon-width);
  }
  .toggle {
    width: 2rem;
  }
</style>
