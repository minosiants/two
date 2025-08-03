<script>
  import Photo from "./Photo.svelte";
  import HeardBroken from "./svg/HeardBroken.svelte";
  import Heard from "./svg/Heard.svelte";
  import Refresh from "./svg/Refresh.svelte";
  import Submit from "./svg/Submit.svelte";
  const { contacts } = $props();

  const random = (max) => Math.floor(Math.random() * max);
  const randomContact = () => contacts[random(contacts.length)];
  let contact = $derived(randomContact());
  let submitted = $state(false);
  let value = $state("");
  const success = () => value === contact.contact;
  let stateClass = $derived(submitted ? (success() ? "success" : "error") : "");
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

<section class="frame box">
  <div class="img stack">
    <ul class="with-sidebar">
      <li class="stack">
        <div class="photo">
          <Photo photo={contact.photo} />
        </div>
        <ul class="stats">
          <li class="cluster">
            <Heard />
            <span>
              {contact.success}
            </span>
          </li>
          <li class="cluster">
            <HeardBroken />
            <span>{contact.fail}</span>
          </li>
        </ul>
      </li>
      <li class="stack">
        <h1>{contact.name}</h1>
        <h3 class={submitted ? "" : "hidden"}>
          {contact.contact}
        </h3>
      </li>
    </ul>
    <form class="with-sidebar">
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
    </form>
  </div>
</section>

<style>
  section {
    --border: 1px solid var(--borderColor-primary);
    background-color: var(--bgColor-second);
    --sidebar-width: var(--s3);
    color: var(--color-second);
    margin-block: auto;
  }
  .img {
    margin-block: auto;
  }

  form.with-sidebar {
    --svg-fill: var(--bgColor-primary);
    --sidebar-gutter: var(--s-1);
    flex-direction: row-reverse;
    justify-content: flex-end;
    --stack-space: var(--s-4);
    align-items: baseline;
  }
  input {
    font-size: var(--font-large);
    font-weight: bold;
  }
  .error {
    /*border: 1px solid red;*/
    color: var(--accentColor-second);
  }
  .hidden {
    visibility: hidden;
  }
  .frame {
    --shadow: 0 3px 6px var(--borderColor-primary),
      0 3px 6px var(--borderColor-primary);
    border: 1px solid var(--borderColor-primary);
    border-radius: var(--border-radius-small);
    box-shadow: var(--shadow);
    --n: 90; /* numerator */
    --d: 70; /* denominator */
  }
  .photo {
    --svg-fill: var(--bgColor-primary);
    --svg-stroke: var(--color-second);
    background-color: var(--ink);
    padding: var(--s-3);
  }
  .stats .cluster {
    flex-wrap: nowrap;
  }
</style>
