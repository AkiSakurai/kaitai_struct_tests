# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from switch_manual_enum import SwitchManualEnum
import switch_manual_enum

class TestSwitchManualEnum(unittest.TestCase):
    def test_switch_manual_enum(self):
        with SwitchManualEnum.from_file('src/switch_opcodes.bin') as r:
            self.assertEqual(len(r.opcodes), 4)
            self.assertEqual(r.opcodes[0].code, switch_manual_enum.SwitchManualEnum.Opcode.CodeEnum.strval)
            self.assertEqual(r.opcodes[0].body.value, u"foobar")
            self.assertEqual(r.opcodes[1].code, switch_manual_enum.SwitchManualEnum.Opcode.CodeEnum.intval)
            self.assertEqual(r.opcodes[1].body.value, 66)
            self.assertEqual(r.opcodes[2].code, switch_manual_enum.SwitchManualEnum.Opcode.CodeEnum.intval)
            self.assertEqual(r.opcodes[2].body.value, 55)
            self.assertEqual(r.opcodes[3].code, switch_manual_enum.SwitchManualEnum.Opcode.CodeEnum.strval)
            self.assertEqual(r.opcodes[3].body.value, u"")
