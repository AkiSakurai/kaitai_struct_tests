# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from str_literals_latin1 import StrLiteralsLatin1

class TestStrLiteralsLatin1(unittest.TestCase):
    def test_str_literals_latin1(self):
        with StrLiteralsLatin1.from_file('src/str_literals_latin1.bin') as r:
            self.assertEqual(r.parsed_eq_literal, True)
