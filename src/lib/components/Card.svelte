<script>
  import Photo from "./Photo.svelte";
  import Check from "./svg/Check.svelte";
  import Plus from "./svg/Plus.svelte";
  import Refresh from "./svg/Refresh.svelte";
  import Submit from "./svg/Submit.svelte";
  import { fade } from "svelte/transition";
  import { getContext } from "svelte";
  const { contacts } = getContext("contacts");

  const random = (max) => Math.floor(Math.random() * max);
  const randomContact = () => contacts[random(contacts.length)];
  const { c = randomContact() } = $props();
  let contact = $derived(c);
  let submitted = $state(false);
  let value = $state("");
  let stateClass = $derived(submitted ? (success() ? "success" : "error") : "");
  const success = () => value === contact.contact;
  $effect(() => (submitted ? (success() ? "success" : "error") : ""));

  const onclick = (e) => {
    submitted = !submitted;
    if (submitted) {
      if (success()) contact.success += 1;
      else contact.fail += 1;
      console.log($state.snapshot(contact));
    }
    if (!submitted) {
      contact = randomContact();
      value = "";
    }
  };
</script>

<section class="center cover">
  <ul class="with-sidebar box frame">
    <li class="stack">
      <div class="photo">
        <Photo photo={contact.photo} />
      </div>
      <div>
        <code>{contact.success}</code>
        <code>{contact.fail}</code>
      </div>
    </li>
    <li class="stack">
      <address>
        {#key contact.name}
          <h2 transition:fade>{contact.name}</h2>
        {/key}
        <h3 class={submitted ? "" : "hidden"} transition:fade>
          {contact.contact}
        </h3>
      </address>
      <form class="">
        <div class="with-sidebar">
          <button {onclick}>
            {#if submitted}
              <Refresh />
            {:else}
              <Submit />
            {/if}
          </button>
          <section class="stack {stateClass}">
            <input type="text" placeholder="phone" bind:value />
            <div class="line"></div>
          </section>
        </div>
      </form>
    </li>
  </ul>
</section>

<form class="">
  <fieldset class="with-sidebar">
    <button>
      <Plus />
    </button>
    <section class="stack">
      <input type="text" placeholder="phone" />
      <div class="line"></div>
    </section>
  </fieldset>
</form>

<style>
  h2 {
    font-size: var(--font-pre-small);
  }
  ul {
    margin-block: auto;
    --sidebar-width: var(--s3);
  }
  form div {
    --sidebar-width: var(--s3);
    --sidebar-gutter: var(--s-1);
    flex-direction: row-reverse;
    justify-content: flex-end;
  }
  .error {
    /*border: 1px solid red;*/
  }
  .success {
    /* border: 1px solid green;*/
  }
  .hidden {
    visibility: hidden;
  }
</style>
