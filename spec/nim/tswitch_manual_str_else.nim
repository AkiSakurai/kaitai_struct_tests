# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/switch_manual_str_else
import auxiliary/test_utils

let r = SwitchManualStrElse.fromFile("src/switch_opcodes2.bin")

assert len(r.opcodes) == int(4)
assert r.opcodes[0].code == string("S")
assert (SwitchManualStrElse_Opcode_Strval(r.opcodes[0].body)).value == string("foo")
assert r.opcodes[1].code == string("X")
assert (SwitchManualStrElse_Opcode_Noneval(r.opcodes[1].body)).filler == uint32(66)
assert r.opcodes[2].code == string("Y")
assert (SwitchManualStrElse_Opcode_Noneval(r.opcodes[2].body)).filler == uint32(51966)
assert r.opcodes[3].code == string("I")
assert (SwitchManualStrElse_Opcode_Intval(r.opcodes[3].body)).value == uint8(7)
