#!/usr/bin/env node
// Not the prettiest script, but it works.

const fs = require('fs');
const path = require('path');

const file = process.argv[2];
const resourcesDir = path.join(__dirname, '..', 'resources');

const yamlContents = fs.readFileSync(file, 'utf8');
const embeddedAscii = yamlContents.replace(/ascii:\ (.*)$/gm, (_match, filename) => {
  const asciiContents = fs.readFileSync(path.join(resourcesDir, filename), 'utf8')
    .replace(/^.*$/gm, line => `    ${line}`);
    return `ascii: |\n${asciiContents}`;
});

console.log(embeddedAscii);
