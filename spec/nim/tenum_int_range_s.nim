# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/enum_int_range_s
import auxiliary/test_utils

let r = EnumIntRangeS.fromFile("src/enum_int_range_s.bin")

assert r.f1 == enum_int_range_s.int_min
assert r.f2 == enum_int_range_s.zero
assert r.f3 == enum_int_range_s.int_max
