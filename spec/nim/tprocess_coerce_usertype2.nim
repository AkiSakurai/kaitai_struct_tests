# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/process_coerce_usertype2
import auxiliary/test_utils

let r = ProcessCoerceUsertype2.fromFile("src/process_coerce_bytes.bin")

assert r.records[0].flag == uint8(0)
assert r.records[0].buf.value == uint32(1094795585)
assert r.records[1].flag == uint8(1)
assert r.records[1].buf.value == uint32(1111638594)
