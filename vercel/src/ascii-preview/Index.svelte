<script lang="ts">
  import Nav from "../lib/Nav.svelte";
  import AsciiPreview from "../lib/AsciiPreview.svelte";
  import data from "../../languages.json";

  const languages =
    // HACK This looks unnecessary to destructure and restructure, but Svelte
    // was freaking out about accessing colors.property in the template.
    Object.entries(data)
      .map(([name, { ascii, colors }]) => ({
        name,
        ascii,
        ...colors,
      }))
      .sort((a, b) => a.name.localeCompare(b.name));
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
      chip={language.chip}
    />
  {/each}
</main>
