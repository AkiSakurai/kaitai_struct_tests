# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from str_literals2 import StrLiterals2

class TestStrLiterals2(unittest.TestCase):
    def test_str_literals2(self):
        with StrLiterals2.from_file('src/fixed_struct.bin') as r:
            self.assertEqual(r.dollar1, u"$foo")
            self.assertEqual(r.dollar2, u"${foo}")
            self.assertEqual(r.hash, u"#{foo}")
            self.assertEqual(r.at_sign, u"@foo")
