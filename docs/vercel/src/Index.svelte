<script lang="ts">
  import AsciiPreview from './lib/AsciiPreview.svelte';
  import data from '../../../languages.yaml';
  import type { Languages, Language } from '../../../languages.yaml';
  import { onMount } from 'svelte';
  import { writable, derived } from 'svelte/store';

  let tagName: string;
  let htmlUrl: string;
  let menuOpen: boolean;

  const languages: Language[] = Object.entries(data as Languages).map(
    ([name, { type, ascii, colors }]) => ({
      name,
      type,
      ascii,
      colors,
    })
  );

  const languageTypes = new Set<string>(
    Object.values(data as Languages).map(({ type }) => type)
  );

  onMount(async () => {
    try {
      const response = await fetch(
        'https://api.github.com/repos/o2sh/onefetch/releases/latest'
      );
      const data = await response.json();

      tagName = data.tag_name;
      htmlUrl = data.html_url;
    } catch (error) {
      console.error('Error:', error);
    }
  });

  const filter = writable({
    checkboxes: [] as string[],
  });

  const filteredLanguages = derived(filter, ($filter) => {
    let filtered: Language[] = languages;
    if ($filter.checkboxes.length > 0) {
      filtered = filtered.filter((d: Language) =>
        $filter.checkboxes.includes(d.type)
      );
    }
    return filtered;
  });
</script>

<header>
  {#if tagName && htmlUrl}
    <div class="banner">
      <small
        >Version {tagName} is available 🎉 Check the
        <a href={htmlUrl}>release notes</a>!!</small>
    </div>
  {/if}
  <h1>Onefetch</h1>
  <p>
    <small>
      <a href="https://github.com/o2sh/onefetch/wiki">wiki</a> |
      <a href="https://github.com/o2sh/onefetch/tree/main/docs/vercel"
        >github</a>
      | Built with ❤ by
      <a href="https://github.com/spenserblack">@spenserblack</a> and
      <a href="https://github.com/o2sh">@o2sh</a></small>
  </p>
</header>
<main>
  <p>
    This page shows you an ASCII preview for all the programming languages
    supported by onefetch. Like the binary, the data is taken from the <a
      href="https://raw.githubusercontent.com/o2sh/onefetch/main/languages.yaml"
      ><code>Languages.yaml</code></a> file and the layout is meant to mimic the
    way the logo would look inside a terminal.
  </p>
  <p>
    Suggestions and PRs are welcome at <a
      href="https://github.com/o2sh/onefetch">github.com/o2sh/onefetch</a>
  </p>
  <h3>Languages <small>({$filteredLanguages.length})</small></h3>

  <button on:click={() => (menuOpen = !menuOpen)}>Filter by type</button>

  <div class:show={!menuOpen} class="checkbox-group">
    {#each languageTypes as type}
      <label for={type}>
        <input
          id={type}
          type="checkbox"
          value={type}
          bind:group={$filter.checkboxes} />
        {type}
      </label>
    {/each}
  </div>

  {#each $filteredLanguages as language}
    <AsciiPreview
      name={language.name}
      ansi={language.colors.ansi}
      hex={language.colors.hex}
      ascii={language.ascii}
      chip={language.colors.chip} />
  {/each}
</main>

<style>
  .banner {
    background-color: #e1f6e5;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    text-align: center;
    padding: 0.5rem 0;
  }

  .checkbox-group {
    margin-top: 1.5rem;
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    text-transform: capitalize;
  }

  .show {
    display: none;
  }
</style>
