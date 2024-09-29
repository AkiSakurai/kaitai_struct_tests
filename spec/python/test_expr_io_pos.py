# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from expr_io_pos import ExprIoPos

class TestExprIoPos(unittest.TestCase):
    def test_expr_io_pos(self):
        with ExprIoPos.from_file('src/expr_io_pos.bin') as r:
            self.assertEqual(r.substream1.my_str, u"CURIOSITY")
            self.assertEqual(r.substream1.body, b"\x11\x22\x33\x44")
            self.assertEqual(r.substream1.number, 66)
            self.assertEqual(r.substream2.my_str, u"KILLED")
            self.assertEqual(r.substream2.body, b"\x61\x20\x63\x61\x74")
            self.assertEqual(r.substream2.number, 103)
