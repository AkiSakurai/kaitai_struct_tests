// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ExprMod', 'src/fixed_struct.bin', function(r) {
  assert.strictEqual(r.intU, 1262698832);
  assert.strictEqual(r.intS, -52947);
  assert.strictEqual(r.modPosConst, 9);
  assert.strictEqual(r.modNegConst, 4);
  assert.strictEqual(r.modPosSeq, 5);
  assert.strictEqual(r.modNegSeq, 2);
});
