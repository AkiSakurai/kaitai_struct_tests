// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ProcessCustom', 'src/process_rotate.bin', function(r, ProcessCustom_) {
  assert.deepStrictEqual(r.buf1, new Uint8Array([16, 179, 148, 148, 244]));
  assert.deepStrictEqual(r.buf2, new Uint8Array([95, 186, 123, 147, 99, 35, 95]));
  assert.deepStrictEqual(r.buf3, new Uint8Array([41, 51, 177, 56, 177]));
});
