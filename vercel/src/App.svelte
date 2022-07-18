<script lang="ts">
  import { onMount } from 'svelte';
  import Counter from './lib/Counter.svelte'
  import getLanguages from './get-languages';
  import type { Languages } from './types';

  let languages: Languages | null = null;

  onMount(async () => {
    languages = await getLanguages();
  });
</script>

<main>
  <h1>Onefetch</h1>

  <div class="card">
    <Counter />
  </div>
  {#if languages}
    <pre>{JSON.stringify(languages, null, 2)}</pre>
  {:else}
    <p>Getting languages</p>
  {/if}
</main>
