# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/switch_repeat_expr
import auxiliary/test_utils

let r = SwitchRepeatExpr.fromFile("../../src/switch_tlv.bin")

assert r.code == 17
assert r.size == 9
assert (SwitchRepeatExpr_One(r.body[0])).first == @[83'u8, 116'u8, 117'u8, 102'u8, 102'u8, 0'u8, 77'u8, 101'u8, 0'u8]
