# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/position_abs
import auxiliary/test_utils

let r = PositionAbs.fromFile("src/position_abs.bin")

assert r.indexOffset == uint32(32)
assert r.index.entry == string("foo")
