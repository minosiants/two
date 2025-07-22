<script>
  import Contact from "./Contact.svelte";
  let { contacts = [] } = $props();
</script>

<ul class="stack">
  {#each contacts as contact}
    <li>
      <Contact id={contact.id} />
    </li>
  {/each}
</ul>
