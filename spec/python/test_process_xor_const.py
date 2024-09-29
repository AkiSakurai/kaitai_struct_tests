# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from process_xor_const import ProcessXorConst

class TestProcessXorConst(unittest.TestCase):
    def test_process_xor_const(self):
        with ProcessXorConst.from_file('src/process_xor_1.bin') as r:
            self.assertEqual(r.key, 255)
            self.assertEqual(r.buf, b"\x66\x6F\x6F\x20\x62\x61\x72")
