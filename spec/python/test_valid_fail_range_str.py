# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from valid_fail_range_str import ValidFailRangeStr
import kaitaistruct

class TestValidFailRangeStr(unittest.TestCase):
    def test_valid_fail_range_str(self):
        with self.assertRaises(kaitaistruct.ValidationGreaterThanError):
            with ValidFailRangeStr.from_file('src/fixed_struct.bin') as r:
                pass
