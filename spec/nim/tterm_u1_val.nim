# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/term_u1_val
import auxiliary/test_utils

let r = TermU1Val.fromFile("src/str_encodings.bin")

assert r.foo == @[10'u8, 0'u8, 83'u8, 111'u8, 109'u8, 101'u8, 32'u8, 65'u8, 83'u8, 67'u8, 73'u8, 73'u8, 15'u8, 0'u8]
assert r.bar == string("\u3053\u3093\u306b")
