# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from valid_fail_eq_int import ValidFailEqInt

class TestValidFailEqInt(unittest.TestCase):
    def test_valid_fail_eq_int(self):
        with ValidFailEqInt.from_file('src/fixed_struct.bin') as r:
            pass
