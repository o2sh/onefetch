<script lang="ts">
  import Chip from './Chip.svelte';

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
          : `<span style="color: ${mapToDefaultTerminalFgColor(
              ansi[i],
              dark
            )}">`;
        spanCount++;
        return spanText;
      });
      return `${htmlLine}${'</span>'.repeat(spanCount)}`;
    })
    .join('\n');

  function mapToDefaultTerminalFgColor(color: string, dark: boolean): string {
    return color === 'white' && !dark ? 'black' : color;
  }
</script>

<div class="title-row">
  <div class="language-name">
    <Chip color={chip} width={24} height={24} />
    <h3>{name}</h3>
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
    background-color: white;
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

  .checkbox {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
  }
</style>
