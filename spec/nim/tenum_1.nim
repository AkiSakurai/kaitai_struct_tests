# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/enum_1
import auxiliary/test_utils

let r = Enum1.fromFile("src/enum_0.bin")

assert r.main.submain.pet1 == enum_1.cat
assert r.main.submain.pet2 == enum_1.chicken
