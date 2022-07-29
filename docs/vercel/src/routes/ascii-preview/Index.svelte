<script lang="ts">
  import Nav from '../../lib/Nav.svelte';
  import AsciiPreview from '../../lib/AsciiPreview.svelte';
  import data from '../../../../../languages.yaml';
  import type { Languages } from '../../../../../languages.yaml';

  const languages = Object.entries(data as Languages).map(
    ([name, { ascii, colors }]) => ({
      name,
      ascii,
      ...colors,
    })
  );
</script>

<main>
  <h1>ASCII Preview</h1>
  <Nav active="ascii-preview/" />
  <h2>Languages ({languages.length})</h2>

  {#each languages as language}
    <AsciiPreview
      name={language.name}
      ansi={language.ansi}
      hex={language.hex}
      ascii={language.ascii}
      chip={language.chip} />
  {/each}
</main>
