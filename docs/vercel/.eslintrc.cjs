module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended'],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 12,
    sourceType: 'module',
    parser: '@typescript-eslint/parser',
    extraFileExtensions: ['.svelte'],
  },
  plugins: ['@typescript-eslint'],
  overrides: [
    {
      files: ['*.svelte'],
      parser: 'svelte-eslint-parser',
    },
  ],
};
