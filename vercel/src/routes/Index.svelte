<script lang="ts">
  import Nav from "../lib/Nav.svelte";
</script>

<main>
  <h1>Onefetch</h1>
  <Nav active="/" />
  <div class="spacer" />
  <p>Get Onefetch from</p>
  <ul>
    <li>
      <a href="https://github.com/o2sh/onefetch/releases/latest"
        >the releases page</a
      >
    </li>
    <li>
      <a href="https://crates.io/crates/onefetch">crates.io</a> (<code
        >cargo install onefetch</code
      >)
    </li>
    <li>
      <a href="https://snapcraft.io/onefetch">snapcraft</a> (<code
        >sudo snap install onefetch</code
      >)
    </li>
  </ul>
  <p>
    or view our
    <a href="https://github.com/o2sh/onefetch/wiki/installation"
      >installation guide</a
    >
    for other installation methods or more details.
  </p>
</main>

<style>
  .spacer {
    margin-top: 5rem;
  }
</style>
