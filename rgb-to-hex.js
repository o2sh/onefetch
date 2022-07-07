#!/usr/bin/env node
const yaml = require('js-yaml');
const fs = require('fs');

const input = fs.readFileSync(process.argv[2], 'utf8');
const src = yaml.load(input);

const asHex = (channel) => channel.toString(16).toUpperCase().padStart(2, '0');

const convertedEntries = Object.entries(src)
  .map(([language, { type, ascii, colors: { ansi, rgb, chip }}]) => {
    const hex = rgb?.map(([r, g, b]) => `#${asHex(r)}${asHex(g)}${asHex(b)}`);
    const hexChip = chip.reduce((s, channel) => `${s}${asHex(channel)}`, '#');
    return [language, { type, ascii, colors: { ansi, hex, chip: hexChip }}];
  });

const converted = yaml.dump(Object.fromEntries(convertedEntries));
console.log(converted);
