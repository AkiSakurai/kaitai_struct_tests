# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/expr_bytes_cmp
import auxiliary/test_utils

let r = ExprBytesCmp.fromFile("src/fixed_struct.bin")

assert r.one == @[80'u8]
assert r.two == @[65'u8, 67'u8, 75'u8]
assert r.isEq == bool(true)
assert r.isNe == bool(false)
assert r.isLt == bool(true)
assert r.isGt == bool(false)
assert r.isLe == bool(true)
assert r.isGe == bool(false)
assert r.isLt2 == bool(false)
assert r.isGt2 == bool(true)
