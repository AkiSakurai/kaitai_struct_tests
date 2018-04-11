// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatUntilComplex', 'src/repeat_until_complex.bin', function(r) {
  assert.strictEqual(r.first.length, 3);
  assert.strictEqual(r.first[0].count, 4);
  assert.deepStrictEqual(r.first[0].values, [(0 + 1), 2, 3, 4]);
  assert.strictEqual(r.first[1].count, 2);
  assert.deepStrictEqual(r.first[1].values, [(0 + 1), 2]);
  assert.strictEqual(r.first[2].count, 0);
  assert.strictEqual(r.second.length, 4);
  assert.strictEqual(r.second[0].count, 6);
  assert.deepStrictEqual(r.second[0].values, [(0 + 1), 2, 3, 4, 5, 6]);
  assert.strictEqual(r.second[1].count, 3);
  assert.deepStrictEqual(r.second[1].values, [(0 + 1), 2, 3]);
  assert.strictEqual(r.second[2].count, 4);
  assert.deepStrictEqual(r.second[2].values, [(0 + 1), 2, 3, 4]);
  assert.strictEqual(r.second[3].count, 0);
  assert.deepStrictEqual(r.third, [(0 + 102), 111, 111, 98, 97, 114, 0]);
});
