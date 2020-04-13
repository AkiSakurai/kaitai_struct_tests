# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils, unittest
import ../../../compiled/nim/process_xor4_const
import ../test_utils

let r = ProcessXor4Const.fromFile("src/process_xor_4.bin")

test "ProcessXor4Const":

  check(r.key == seq[byte](@[-20'u8, -69, -93, 20]))
  check(r.buf == seq[byte](@[102'u8, 111, 111, 32, 98, 97, 114]))
  discard
