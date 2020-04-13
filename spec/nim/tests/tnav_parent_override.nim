# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils, unittest
import ../../../compiled/nim/nav_parent_override
import ../test_utils

let r = NavParentOverride.fromFile("src/nav_parent_codes.bin")

test "NavParentOverride":

  check(r.childSize == uint8(3))
  check(r.child1.data == seq[byte](@[73'u8, 49, 50]))
  check(r.mediator2.child2.data == seq[byte](@[51'u8, 66, 98]))
  discard
