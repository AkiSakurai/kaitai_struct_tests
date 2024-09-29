# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from expr_to_i_trailing import ExprToITrailing
import kaitaistruct

class TestExprToITrailing(unittest.TestCase):
    def test_expr_to_i_trailing(self):
        with ExprToITrailing.from_file('src/term_strz.bin') as r:
            with self.assertRaises(ValueError):
                r.to_i_r10
            self.assertEqual(r.to_i_r16, 152517308)
            with self.assertRaises(ValueError):
                r.to_i_garbage
