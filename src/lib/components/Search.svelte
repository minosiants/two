<script>
  import Contacts from "./Contacts.svelte";
  import ArrowLeft from "./svg/ArrowLeft.svelte";
  import Magnifier from "./svg/Magnifier.svelte";

  let { contacts = $bindable() } = $props();

  let focus = $state(false);
  let result = $state([]);
  const onfocus = () => {
    focus = true;
  };

  const onclick = () => {
    result = [];
    focus = false;
  };
  const oninput = (v) => {
    const i = v.target.value.trim().toLowerCase();
    result =
      i.length === 0
        ? []
        : contacts.filter((c) => c.name.toLowerCase().search(i) !== -1);
  };
</script>

{#if focus != true}
  <section class="stack">
    <form class="with-sidebar">
      <Magnifier />
      <input {onfocus} placeholder="Search contacts" name="search" />
    </form>
    <div class="line"></div>
  </section>
{:else}
  <section class="stack active">
    <form class="with-sidebar box">
      <label {onclick} for="search">
        <ArrowLeft />
      </label>
      <input placeholder="Search contacts" name="search" {oninput} />
    </form>
    <div class="box">
      <div class="result">
        <Contacts contacts={result} />
      </div>
    </div>
  </section>
{/if}

<style>
  section {
    --sidebar-gutter: var(--s1);
    --sidebar-width: 2rem;
    --stack-space: var(--s-1);
    --line-color: var(--bgColor-primary-light);
    transition: all 1s ease-out;
    color: var(--color-primary);
  }
  section.active {
    --svg-fill: var(--bgColor-primary);
    background-color: var(--bgColor-primary-light);
    position: absolute;
    left: 0;
    top: 0;
    height: 100vh;
    width: 100%;
    z-index: 10;
  }
  section :not(.active) {
    --svg-fill: var(--bgColor-primary-light);
  }
  .active form {
    background-color: var(--bgColor-primary-light);
  }
  input {
    font-size: var(--font-size-h6);
  }
  .result {
    height: 100vh;
  }
</style>
