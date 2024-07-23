# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from term_strz_utf16_v2 import _schema

class TestTermStrzUtf16V2(unittest.TestCase):
    def test_term_strz_utf16_v2(self):
        r = _schema.parse_file('src/term_strz_utf16.bin')

        self.assertEqual(r.s1, u"a\u0200b")
        self.assertEqual(r.s2, u"c\u0200d\000")
        self.assertEqual(r.s3, u"e\u0200f")
