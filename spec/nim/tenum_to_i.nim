# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/enum_to_i
import auxiliary/test_utils

let r = EnumToI.fromFile("../../src/enum_0.bin")

assert r.pet1 == enum_to_i.cat
assert r.pet2 == enum_to_i.chicken
assert r.pet1I == 7
assert r.pet1Mod == 32775
assert r.oneLtTwo == true
assert r.pet1EqInt == true
assert r.pet2EqInt == false
