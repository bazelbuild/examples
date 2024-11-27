const assert = require('assert');
const fs = require('fs');

// Make sure there's a file like react-router/build/server/index.js
const files = fs.readdirSync('react-router/build/server');
console.log(files);
assert.ok(files.some((f) => /index\.js/.test(f)));