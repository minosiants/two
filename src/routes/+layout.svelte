<script>
  import Menu from "$lib/components/Menu.svelte";
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
