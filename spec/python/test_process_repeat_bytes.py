# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from process_repeat_bytes import ProcessRepeatBytes

class TestProcessRepeatBytes(unittest.TestCase):
    def test_process_repeat_bytes(self):
        with ProcessRepeatBytes.from_file('src/process_xor_4.bin') as r:
            self.assertEqual(r.bufs[0], b"\x72\x25\x3D\x8A\x14")
            self.assertEqual(r.bufs[1], b"\x4A\x52\xAA\x10\x44")
