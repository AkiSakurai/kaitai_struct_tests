# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from process_coerce_bytes import ProcessCoerceBytes

class TestProcessCoerceBytes(unittest.TestCase):
    def test_process_coerce_bytes(self):
        with ProcessCoerceBytes.from_file('src/process_coerce_bytes.bin') as r:
            self.assertEqual(r.records[0].flag, 0)
            self.assertEqual(r.records[0].buf, b"\x41\x41\x41\x41")
            self.assertEqual(r.records[1].flag, 1)
            self.assertEqual(r.records[1].buf, b"\x42\x42\x42\x42")
