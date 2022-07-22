<script lang="ts">
  import Chip from './Chip.svelte';
  import type { LanguageColors } from '../types';

  export let name: string;
  export let ansi: string[];
  export let hex: string[] | null = null;
  export let chip: string;
  export let ascii: string = '';

  let dark = true;
  let trueColor = hex != null;

  $: html = ascii
    .split('\n')
    .map((line) => {
      // TODO Clean up, this is hard to read
      let spanCount = 0;
      const htmlLine = line.replace(/\{(\d+)\}/g, (_match, index) => {
        const i = Number.parseInt(index, 10);
        const spanText = trueColor
          ? `<span style="color: ${hex[i]}">`
          : `<span style="color: ${ansi[i]}">`;
        spanCount++;
        return spanText;
      });
      return `${htmlLine}${'</span>'.repeat(spanCount)}`;
    })
    .join('\n');
</script>

<div class="title-row">
  <!-- TODO Fix a11y warning for form label + control -->
   <div class="language-name">
    <Chip color={chip} width={24} height={24} />
    <h3>{name}</h3>
  </div>
  <label>
    <input type="checkbox" bind:checked={dark} />
    Dark
  </label>
  <label>
    {#if hex != null}
      <input type="checkbox" bind:checked={trueColor} />
      True Color
    {:else}
      <input type="checkbox" disabled />
      <span class="text-light">True Color</span>
    {/if}
  </label>
</div>
<pre class:dark>{@html html}</pre>

<style>
  .title-row {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: baseline;
  }

  pre {
    line-height: 1.1;
  }

  pre.dark {
    background-color: black;
  }

  .language-name {
    display: flex;
    flex-direction: row;
    align-items: baseline;
    gap: 1.5rem;
  }
</style>
