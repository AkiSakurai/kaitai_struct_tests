# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from bits_unaligned_b32_be import BitsUnalignedB32Be

class TestBitsUnalignedB32Be(unittest.TestCase):
    def test_bits_unaligned_b32_be(self):
        with BitsUnalignedB32Be.from_file('src/process_xor_4.bin') as r:
            self.assertEqual(r.a, True)
            self.assertEqual(r.b, 3648472617)
            self.assertEqual(r.c, 10)
