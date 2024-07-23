// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TermStrzUtf16V3', 'src/term_strz_utf16.bin', function(r, TermStrzUtf16V3) {

  assert.strictEqual(r.s1, "a\u0200b");
  assert.strictEqual(r.term, 0);
  assert.strictEqual(r.s2, "c\u0200d");
  assert.strictEqual(r.s3, "");
});
