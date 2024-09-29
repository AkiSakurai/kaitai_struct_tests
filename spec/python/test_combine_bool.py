# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from combine_bool import CombineBool

class TestCombineBool(unittest.TestCase):
    def test_combine_bool(self):
        with CombineBool.from_file('src/enum_negative.bin') as r:
            self.assertEqual(r.bool_bit, True)
            self.assertEqual(r.bool_calc_bit, False)
