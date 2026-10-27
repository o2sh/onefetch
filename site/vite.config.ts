import adapter from '@sveltejs/adapter-auto';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import yaml from '@rollup/plugin-yaml';

export default defineConfig({
  plugins: [sveltekit({ adapter: adapter() }), yaml()]
});
