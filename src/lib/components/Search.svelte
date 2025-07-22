<script>
  import Contacts from "./Contacts.svelte";
  import ArrowLeft from "./svg/ArrowLeft.svelte";
  import Magnifier from "./svg/Magnifier.svelte";
  import { getContext } from "svelte";
  let contacts = getContext("contacts");
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
      <label for="search">
        <Magnifier />
      </label>
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
    --color: var(--dark-lighter);
    --svg-fill: var(--color);
    --sidebar-gutter: var(--s1);
    --stack-space: var(--s-1);
    transition: all 1s ease-out;
  }
  section.active {
    background-color: inherit;
    position: absolute;
    left: 0;
    top: 0;
    height: 100svh;
    width: 100%;
    z-index: 10;
  }
  .active form {
    background-color: var(--accent);
  }
  input {
    font-size: var(--font-size-h6);
  }
  .line {
    background-color: var(--color);
    height: var(--s-5);
    transition: background-color 0.3s;
  }
  .result {
    height: 100vh;
  }
</style>
