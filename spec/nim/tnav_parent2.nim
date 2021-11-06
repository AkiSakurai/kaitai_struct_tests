# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/nav_parent2
import auxiliary/test_utils

let r = NavParent2.fromFile("src/nav_parent2.bin")

assert r.ofsTags == uint32(8)
assert r.numTags == uint32(2)
assert r.tags[0].name == string("RAHC")
assert r.tags[0].ofs == uint32(32)
assert r.tags[0].numItems == uint32(3)
assert r.tags[0].tagContent.content == string("foo")
assert r.tags[1].name == string("RAHC")
assert r.tags[1].ofs == uint32(35)
assert r.tags[1].numItems == uint32(6)
assert r.tags[1].tagContent.content == string("barbaz")
