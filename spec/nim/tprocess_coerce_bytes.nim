# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/process_coerce_bytes
import auxiliary/test_utils

let r = ProcessCoerceBytes.fromFile("../../src/process_coerce_bytes.bin")

assert r.records[0].flag == 0
assert r.records[0].buf == @[65'u8, 65'u8, 65'u8, 65'u8]
assert r.records[1].flag == 1
assert r.records[1].buf == @[66'u8, 66'u8, 66'u8, 66'u8]
