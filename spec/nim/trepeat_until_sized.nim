# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/repeat_until_sized
import auxiliary/test_utils

let r = RepeatUntilSized.fromFile("src/repeat_until_process.bin")

assert len(r.records) == int(3)
assert r.records[0].marker == uint8(232)
assert r.records[0].body == uint32(2863311546'i64)
assert r.records[1].marker == uint8(250)
assert r.records[1].body == uint32(2863315102'i64)
assert r.records[2].marker == uint8(170)
assert r.records[2].body == uint32(1431655765)
