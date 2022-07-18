<script lang="ts">
  import type { LanguageColors } from '../types';

  export let name: string;
  export let ansi: string[];
  export let hex: string[] | null = null;
  export let ascii: string = '';

  let dark = true;
  let trueColor = hex != null;

  $: html = ascii.split('\n').join('\n');
</script>

<div class="title-row">
  <h3>{name}</h3>
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

  pre.dark {
    background-color: #4a4a4a;
    color: #c9c9c9;
  }

  .text-light {
    color: gray;
  }
</style>
