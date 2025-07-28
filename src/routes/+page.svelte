<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import contacts from "$lib/js/contacts";
  import { invoke } from "@tauri-apps/api/core";
  import { setContext } from "svelte";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
  let contact = $state({
    id: 1,
    photo: "",
    selected: false,
    name: "a Kaspar Minosyants",
    contact: "+71234567",
  });

  setContext("contacts", contacts);
</script>

<main class="">
  <Card />
</main>

<style>
</style>
