#!/usr/bin/env node
const langs = require('../languages.json');
const reorderedEntries = Object.entries(langs).map(([lang, { name, type, ascii, colors, serialization }]) => [
  lang,
  {
    name,
    type,
    ascii,
    colors: {
      ansi: colors.ansi,
      rgb: colors.rgb ?? undefined,
    },
    serialization: serialization ?? undefined,
  },
]);
console.log(JSON.stringify(Object.fromEntries(reorderedEntries), null, 2));
