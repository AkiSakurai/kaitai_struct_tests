# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/expr_3
import auxiliary/test_utils

let r = Expr3.fromFile("src/fixed_struct.bin")

assert r.one == uint8(80)
assert r.two == string("ACK")
assert r.three == string("@ACK")
assert r.four == string("_ACK_")
assert r.isStrEq == bool(true)
assert r.isStrNe == bool(false)
assert r.isStrLt == bool(true)
assert r.isStrGt == bool(false)
assert r.isStrLe == bool(true)
assert r.isStrGe == bool(false)
assert r.isStrLt2 == bool(true)
assert r.testNot == bool(true)
