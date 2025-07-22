<script>
  import Contacts from "$lib/components/Contacts.svelte";
  import Search from "$lib/components/Search.svelte";
  import { setContext } from "svelte";
  const _contacts = [
    {
      id: 1,
      photo: "",
      selected: false,
      name: "a Kaspar Minosyants",
      contact: "+71234567",
    },
    {
      id: 2,
      photo: "",
      selected: false,
      name: "d Kaspar Minosyants",
      contact: "+71234567",
    },
    {
      id: 3,
      photo: "",
      selected: false,
      name: "c Kaspar Minosyants",
      contact: "+71234567",
    },
    {
      id: 4,
      photo: "",
      selected: false,
      name: "Kaspar Minosyants",
      contact: "+71234567",
    },
    {
      id: 5,
      photo: "",
      selected: false,
      name: "Kaspar Minosyants",
      contact: "+71234567",
    },
    {
      id: 6,
      photo: "",
      selected: false,
      name: "Kaspar Minosyants",
      contact: "+71234567",
    },
    {
      id: 7,
      photo: "",
      selected: false,
      name: "Kaspar Minosyants",
      contact: "+71234567",
    },
    {
      id: 8,
      photo: "",
      selected: false,
      name: "Kaspar Minosyants",
      contact: "+71234567",
    },
  ];
  let contacts = $state(_contacts);
  setContext("contacts", contacts);
</script>

<main class="stack box">
  <Search />
  <Contacts {contacts} />
</main>

<style>
</style>
