<script>
  import { setContext } from "svelte";
  import Contact from "./Contact.svelte";
  let { contacts = $bindable() } = $props();
</script>

<ul class="stack">
  {#each contacts as contact, i}
    <li>
      <Contact bind:contact={contacts[i]} />
    </li>
  {/each}
</ul>
