# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/bits_signed_b32_le
import auxiliary/test_utils

let r = BitsSignedB32Le.fromFile("src/bits_signed_b32_le.bin")

assert r.aNum == uint64(0)
assert r.aBit == bool(true)
assert r.bNum == uint64(2147483647)
assert r.bBit == bool(false)
