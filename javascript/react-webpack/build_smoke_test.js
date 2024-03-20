const assert = require('assert');
const fs = require('fs');

// Make sure there's a file like bundle/bundle.js *the mane of the file is controlled by the webpack.bazel.config.js
const files = fs.readdirSync('react-webpack/bundle');
console.log(files);
assert.ok(files.some((f) => /bundle.js/.test(f)));
