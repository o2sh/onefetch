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
        const spanText = `<span style="font-weight: bold; color: ${
          trueColor ? hex[i] : mapToDefaultTerminalFgColor(ansi[i], dark)
        }">`;
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
    <a class="anchor" id={name} href="#{name}">
      <svg
        class="octicon octicon-link"
        viewBox="0 0 16 24"
        version="1.1"
        width="16"
        height="24"
        aria-hidden="true"
        ><path
          fill-rule="evenodd"
          d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z" /></svg>
    </a>
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
<div class="logo-container" class:dark>
  <pre class:dark>{@html html}</pre>
</div>

<style>
  .logo-container {
    display: flex;
    justify-content: center;
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

  .anchor {
    float: left;
    padding-right: 4px;
    margin-left: -20px;
    line-height: 1;
  }

  .title-row .anchor {
    visibility: hidden;
  }

  .title-row:hover .anchor {
    visibility: visible;
  }

  .title-row a:hover {
    border-bottom: 0;
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
