<script lang="ts">
  import Nav from '../lib/Nav.svelte'
  import AsciiPreview from '../lib/AsciiPreview.svelte';
  import type { Languages } from '../types';
  import  getLanguages  from '../get-languages';

  const languages = (async () => {
    const languageObject = await getLanguages();
    // HACK This looks unnecessary to destructure and restructure, but Svelte
    // was freaking out about accessing colors.property in the template.
    return Object.entries(languageObject).map(([name, { ascii, colors }]) => ({
      name,
      ascii,
      ...colors,
    }))
    .sort((a, b) => a.name.localeCompare(b.name));
  })();
</script>

<main>
  <h1>ASCII Preview</h1>

  <div class="card">
    <Nav active="/ascii-preview" />
  </div>
  <h2>Languages</h2>
  {#await languages}
    <p>Fetching languages...</p>
  {:then result}
    {#each result as language}
      <AsciiPreview
        name={language.name}
        ansi={language.ansi}
        hex={language.hex}
        ascii={language.ascii}
        chip={language.chip}
      />
    {/each}
  {:catch error}
    <p class="danger">
      <strong>Couldn't fetch languages:</strong>
      <code>{error.message}</code>
    </p>
  {/await}
</main>

<style>
  .danger {
    color: red;
  }
</style>
