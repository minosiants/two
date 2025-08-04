<script>
  import Menu from "$lib/components/Menu.svelte";
  import { getContacts, saveContacts } from "$lib/js/commands";
  import { readContacts } from "$lib/js/fs";
  import { contactsStore } from "$lib/js/store";
  import { onMount } from "svelte";
  onMount(async () =>
    readContacts()
      .then((c) => {
        if (c.length === 0) return getContacts();
        else {
          console.log("c", c);
          return c;
        }
      })
      .then((contacts) => {
        contactsStore.set(contacts);
        console.log("::::::::::::", contacts);
      }),
  );
</script>

<svelte:head>
  <link rel="stylesheet" href="/css/vars.css" />
  <link rel="stylesheet" href="/css/root.css" />
  <link rel="stylesheet" href="/css/layout.css" />
  <link rel="stylesheet" href="/css/fonts.css" />
  <link rel="stylesheet" href="/css/components.css" />
  <link rel="stylesheet" href="/css/utils.css" />
  <link rel="stylesheet" href="/css/form.css" />
  <link rel="stylesheet" href="/css/typography.css" />
</svelte:head>

<main class="layout">
  <slot />
  <Menu />
</main>

<style>
</style>
