# Autogenerated from KST: please remove this line if doing any edits by hand!

import ../../compiled/nim/vlq_base128_le
import os, streams, options, sequtils
import ../../compiled/nim/imports_abs
import auxiliary/test_utils

let r = ImportsAbs.fromFile("src/fixed_struct.bin")

assert r.len.value == int(80)
assert len(r.body) == int(80)
