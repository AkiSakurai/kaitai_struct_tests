# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/yaml_ints
import auxiliary/test_utils

let r = YamlInts.fromFile("src/fixed_struct.bin")

assert r.testU4Dec == int(4294967295'i64)
assert r.testU4Hex == int(4294967295'i64)
assert r.testU8Dec == int(18446744073709551615'u64)
assert r.testU8Hex == int(18446744073709551615'u64)
