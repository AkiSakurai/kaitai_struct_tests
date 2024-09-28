// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ExprBytesCmp', 'src/fixed_struct.bin', function(r, ExprBytesCmp_) {
  assert.deepStrictEqual(r.one, new Uint8Array([80]));
  assert.deepStrictEqual(r.two, new Uint8Array([65, 67, 75]));
  assert.strictEqual(r.isEq, true);
  assert.strictEqual(r.isNe, false);
  assert.strictEqual(r.isLt, true);
  assert.strictEqual(r.isGt, false);
  assert.strictEqual(r.isLe, true);
  assert.strictEqual(r.isGe, false);
  assert.strictEqual(r.isLt2, false);
  assert.strictEqual(r.isGt2, true);
});
