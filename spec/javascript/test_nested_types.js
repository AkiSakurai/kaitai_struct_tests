// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NestedTypes', 'src/fixed_struct.bin', function(r) {
  assert.strictEqual(r.one.typedAtRoot.valueB, 80);
  assert.strictEqual(r.one.typedHere.valueC, 65);
  assert.strictEqual(r.two.valueB, 67);
});
