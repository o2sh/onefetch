<script lang="ts">
  import { mapToDefaultTerminalFgColor } from '../lib/utils';
  import TitleLink from './TitleLink.svelte';

  export let name: string;
  export let ansi: string[];
  export let hex: string[] | null = null;
  export let chipColor: string;
  export let ascii: string = '';
  export let chipIcon: string;

  let dark = true;
  let trueColor = hex != null;

  $: html = ascii
    .split('\n')
    .map((line) => {
      // TODO Clean up, this is hard to read
      let spanCount = 0;
      const htmlLine = line.replace(/\{(\d+)\}/g, (_match, index) => {
        const i = Number.parseInt(index, 10);
        const spanText = `<span style="font-weight: bold; color: ${
          trueColor && hex ? hex[i] : mapToDefaultTerminalFgColor(ansi[i], dark)
        }">`;
        spanCount++;
        return spanText;
      });
      return `${htmlLine}${'</span>'.repeat(spanCount)}`;
    })
    .join('\n');
</script>

<div class="title-row">
  <div class="language-name">
    <h3 class="nerd-font" style="color: {chipColor}">{chipIcon}</h3>
    <TitleLink {name} />
  </div>
  <div class="checkbox">
    <input id="dark-checkbox-{name}" type="checkbox" bind:checked={dark} />
    <label for="dark-checkbox-{name}">Dark</label>
  </div>
  <div class="checkbox">
    <input
      id="hex-checkbox-{name}"
      type="checkbox"
      disabled={hex == null}
      bind:checked={trueColor} />
    <label for="hex-checkbox-{name}">True Color</label>
  </div>
</div>
<div class="logo-container" class:dark>
  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  <pre class:dark>{@html html}</pre>
</div>

<style>
  @import url('https://www.nerdfonts.com/assets/css/webfont.css');

  .logo-container {
    display: flex;
    justify-content: center;
    background-color: white;
  }

  .logo-container.dark {
    background-color: black;
  }

  .title-row {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: baseline;
  }

  pre {
    line-height: 1.1;
    background-color: white;
    display: inline-block;
    margin-bottom: 0px;
  }

  pre.dark {
    background-color: black;
  }

  .language-name {
    display: flex;
    flex-direction: row;
    align-items: baseline;
    gap: 0.5rem;
  }

  .checkbox {
    display: flex;
    align-items: baseline;
    gap: 0.1rem;
  }
</style>
