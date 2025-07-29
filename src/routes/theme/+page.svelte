<script>
  import Card from "$lib/components/Card.svelte";
  import Colors from "$lib/components/Colors.svelte";
  import contacts from "$lib/js/contacts";
  import { setContext } from "svelte";

  setContext("contacts", contacts);
</script>

<section>
  <div class=" box">
    <Card />
  </div>
</section>
<section>
  <Colors />
</section>
<section class="logo">
  <div>Two</div>
</section>

<style>
  .logo {
    --color: var(--accentColor-primary);
    background: var(--bgColor-primary);
    color: var(--color);
    padding: 4rem;
    margin: 4rem;
    font-size: 4rem;
    border: 2px solid var(--color);
  }
</style>
