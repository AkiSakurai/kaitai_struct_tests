# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/cast_to_top
import auxiliary/test_utils

let r = CastToTop.fromFile("src/fixed_struct.bin")

assert r.code == uint8(80)
assert r.header.code == uint8(65)
assert r.headerCasted.code == uint8(65)
