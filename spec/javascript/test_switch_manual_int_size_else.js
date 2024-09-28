// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchManualIntSizeElse', 'src/switch_tlv.bin', function(r, SwitchManualIntSizeElse_) {
  assert.strictEqual(r.chunks.length, 4);
  assert.strictEqual(r.chunks[0].code, 17);
  assert.strictEqual(r.chunks[0].body.title, "Stuff");
  assert.strictEqual(r.chunks[0].body.author, "Me");
  assert.strictEqual(r.chunks[1].code, 34);
  assert.deepStrictEqual(r.chunks[1].body.entries, ["AAAA", "BBBB", "CCCC"]);
  assert.strictEqual(r.chunks[2].code, 51);
  assert.deepStrictEqual(r.chunks[2].body.rest, new Uint8Array([16, 32, 48, 64, 80, 96, 112, 128]));
  assert.strictEqual(r.chunks[3].code, 255);
  assert.deepStrictEqual(r.chunks[3].body.rest, new Uint8Array([]));
});
