# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/bits_unaligned_b64_be
import auxiliary/test_utils

let r = BitsUnalignedB64Be.fromFile("../../src/process_xor_4.bin")

assert r.a == true
assert r.b == 15670070570729969769'u64
assert r.c == 14
