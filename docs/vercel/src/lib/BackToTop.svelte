<script>
  export let showOnPx = 150;
  let hidden = true;

  function goTop() {
    document.body.scrollIntoView();
  }

  function scrollContainer() {
    return document.documentElement || document.body;
  }

  function handleOnScroll() {
    if (!scrollContainer()) {
      return;
    }

    if (scrollContainer().scrollTop > showOnPx) {
      hidden = false;
    } else {
      hidden = true;
    }
  }
</script>

<svelte:window on:scroll={handleOnScroll} />

<div
  class="back-to-top"
  id="hide-on-small-display"
  on:click={goTop}
  class:hidden>
  ⬆
</div>

<style>
  .back-to-top {
    opacity: 1;
    transition: opacity 0.5s, visibility 0.5s;
    position: fixed;
    right: 20px;
    bottom: 20px;
    color: white;
    padding: 10px;
    border-radius: 10px;
    background-color: black;
    cursor: pointer;
  }
  @media only screen and (max-width: 800px) {
    #hide-on-small-display {
      opacity: 0;
      visibility: hidden;
    }
  }
  .back-to-top.hidden {
    opacity: 0;
    visibility: hidden;
  }
</style>
