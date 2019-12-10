# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from enum_long_range_s import _schema

class TestEnumLongRangeS(unittest.TestCase):
    def test_enum_long_range_s(self):
        r = _schema.parse_file('src/enum_long_range_s.bin')

        self.assertEqual(r.f1, self._root.Constants.long_min)
        self.assertEqual(r.f2, self._root.Constants.int_below_min)
        self.assertEqual(r.f3, self._root.Constants.int_min)
        self.assertEqual(r.f4, self._root.Constants.zero)
        self.assertEqual(r.f5, self._root.Constants.int_max)
        self.assertEqual(r.f6, self._root.Constants.int_over_max)
        self.assertEqual(r.f7, self._root.Constants.long_max)
