<script lang="ts">
  import { mapToDefaultTerminalFgColor } from '../lib/utils';
  import toast from 'svelte-hot-french-toast';
  import TitleLink from './TitleLink.svelte';

  export let name: string;
  export let ansi: string[];
  export let hex: string[] | null = null;
  export let chipColor: string;
  export let ascii: string = '';
  export let chipIcon: string;

  let dark = true;
  let trueColor = hex != null;

  const copyAscii = async () => {
    const plainAscii = ascii.replace(/\{\d+\}/g, '');

    try {
      await navigator.clipboard.writeText(plainAscii);
      toast.success('Copied to clipboard');
    } catch {
      toast.error('Could not copy to clipboard');
    }
  };

  $: html = ascii
    .split('\n')
    .map((line) => {
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

<div class="language-name">
  <h3 class="nerd-font" style="color: {chipColor}">{chipIcon}</h3>
  <TitleLink {name} />
</div>

<div class="logo-container" class:dark>
  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  <pre class:dark>{@html html}</pre>
</div>

<div class="controls">
  <div class="checkboxes">
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
  <button
    class="copy-button"
    type="button"
    on:click={copyAscii}
    aria-label="Copy ASCII to clipboard">
    Copy
  </button>
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
  }

  .checkboxes {
    display: flex;
    gap: 1rem;
  }

  .controls {
    margin-top: 0.5rem;
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }
</style>
