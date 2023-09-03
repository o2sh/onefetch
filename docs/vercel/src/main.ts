import Index from './Index.svelte';

const targetElement = document.getElementById('app');

if (!targetElement) {
  throw new Error("Couldn't find the 'app' element!");
}

const app = new Index({
  target: targetElement,
});

export default app;
