# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from valid_short import ValidShort

class TestValidShort(unittest.TestCase):
    def test_valid_short(self):
        with ValidShort.from_file('src/fixed_struct.bin') as r:
            pass
