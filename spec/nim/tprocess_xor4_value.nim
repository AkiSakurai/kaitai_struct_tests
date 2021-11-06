# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/process_xor4_value
import auxiliary/test_utils

let r = ProcessXor4Value.fromFile("src/process_xor_4.bin")

assert r.key == @[-20'u8, -69'u8, -93'u8, 20'u8]
assert r.buf == @[102'u8, 111'u8, 111'u8, 32'u8, 98'u8, 97'u8, 114'u8]
