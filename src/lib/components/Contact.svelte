<script>
  import Photo from "./Photo.svelte";
  import ToggleBtn from "./ToggleBtn.svelte";
  import { getContext } from "svelte";
  const { id } = $props();
  let contact = getContext("contacts").find((v) => v.id === id);
</script>

<article class="cluster">
  <div class="cluster">
    <Photo photo={contact.photo} />
    <ul class="stack">
      <li>{contact.name}</li>
      <li>{contact.contact}</li>
    </ul>
  </div>
  <ToggleBtn {id} />
</article>

<style>
  article {
    --cluster-justify: space-between;
    --stack-space: var(--s-3);
  }
  ul :first-child {
    font-weight: bold;
  }
</style>
