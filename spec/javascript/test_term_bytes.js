// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TermBytes', 'src/term_strz.bin', function(r, TermBytes_) {
  assert.deepStrictEqual(r.s1, new Uint8Array([102, 111, 111]));
  assert.deepStrictEqual(r.s2, new Uint8Array([98, 97, 114]));
  assert.deepStrictEqual(r.s3, new Uint8Array([124, 98, 97, 122, 64]));
});
