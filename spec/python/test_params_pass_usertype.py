# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from params_pass_usertype import ParamsPassUsertype

class TestParamsPassUsertype(unittest.TestCase):
    def test_params_pass_usertype(self):
        with ParamsPassUsertype.from_file('src/position_in_seq.bin') as r:
            self.assertEqual(r.first.foo, 1)
            self.assertEqual(r.one.buf, b"\x02")
