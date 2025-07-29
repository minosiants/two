<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Colors from "$lib/components/Colors.svelte";
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
  setContext("contacts", contacts);
</script>

<main class="center cover">
  <Card />
</main>

<style>
</style>
