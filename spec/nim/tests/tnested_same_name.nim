# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils, unittest
import ../../../compiled/nim/nested_same_name
import ../test_utils

let r = NestedSameName.fromFile("src/repeat_n_struct.bin")

test "NestedSameName":

  check(r.mainData.mainSize == int32(2))
  check(r.mainData.foo.data == seq[byte](@[16'u8, 0, 0, 0]))
  discard
