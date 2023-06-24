<script lang="ts">
  import AsciiPreview from './lib/AsciiPreview.svelte';
  import data from '../../../languages.yaml';
  import type { Languages } from '../../../languages.yaml';
  import { onMount } from 'svelte';

  let tag_name: string;
  let html_url: string;

  const languages = Object.entries(data as Languages).map(
    ([name, { ascii, colors }]) => ({
      name,
      ascii,
      ...colors,
    })
  );

  onMount(async () => {
    try {
      const response = await fetch(
        'https://api.github.com/repos/o2sh/onefetch/releases/latest'
      );
      const data = await response.json();

      tag_name = data.tag_name;
      html_url = data.html_url;
    } catch (error) {
      console.error('Error:', error);
    }
  });
</script>

<header>
  {#if tag_name && html_url}
    <div class="banner">
      <small
        >Version {tag_name} is available 🎉 Check the
        <a href={html_url}>release notes</a>!!</small>
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
  <h3>Languages <small>({languages.length})</small></h3>
  {#each languages as language}
    <AsciiPreview
      name={language.name}
      ansi={language.ansi}
      hex={language.hex}
      ascii={language.ascii}
      chip={language.chip} />
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
</style>
