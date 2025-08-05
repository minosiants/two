<script>
  import { saveContacts, contactsState } from "$lib/js/contacts.svelte";
  import Contacts from "./svg/Contacts.svelte";
  import Play from "./svg/Play.svelte";

  const onclick = async () => {
    saveContacts(contactsState.value);
  };
</script>

<nav class="">
  <a href="/" {onclick}>
    <Play />
  </a>

  <a href="/contacts" {onclick}>
    <Contacts />
  </a>
</nav>

<style>
  nav {
    --svg-fill: var(--bgColor-primary);
    --svg-stroke: none;
    position: fixed;
    left: 0;
    right: 0;
    bottom: 0;
    height: var(--s3);
    background-color: var(--bgColor-primary-light);
    display: flex;
    justify-content: space-around;
    align-items: center;
    z-index: 10;
  }
  a {
    width: var(--s2);
  }
</style>
