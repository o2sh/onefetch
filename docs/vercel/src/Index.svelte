<script lang="ts">
  import AsciiPreview from './lib/AsciiPreview.svelte';
  import data from '../../../languages.yaml';
  import type { Languages } from '../../../languages.yaml';

  const languages = Object.entries(data as Languages).map(
    ([name, { ascii, colors }]) => ({
      name,
      ascii,
      ...colors,
    })
  );
</script>

<header>
  <div class="banner">
    <small
      >Version 2.16 is available 🎉 Check the <a
        href="https://github.com/o2sh/onefetch/releases/tag/2.16.0"
        >release notes</a
      >!!</small>
  </div>
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
