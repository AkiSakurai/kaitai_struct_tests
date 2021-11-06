# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/float_to_i
import auxiliary/test_utils

let r = FloatToI.fromFile("src/floating_points.bin")

assert r.singleValue == float32(0.5)
assert r.doubleValue == float64(0.25)
assert r.singleI == int(0)
assert r.doubleI == int(0)
assert r.float1I == int(1)
assert r.float2I == int(1)
assert r.float3I == int(1)
assert r.float4I == int(-2)
