# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/default_bit_endian_mod
import auxiliary/test_utils

let r = DefaultBitEndianMod.fromFile("src/fixed_struct.bin")

assert r.main.one == uint64(336)
assert r.main.two == uint64(8608)
assert r.main.nest.two == uint64(11595)
assert r.main.nestBe.two == uint64(12799)
