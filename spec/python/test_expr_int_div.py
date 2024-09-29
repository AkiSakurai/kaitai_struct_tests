# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from expr_int_div import ExprIntDiv

class TestExprIntDiv(unittest.TestCase):
    def test_expr_int_div(self):
        with ExprIntDiv.from_file('src/fixed_struct.bin') as r:
            self.assertEqual(r.int_u, 1262698832)
            self.assertEqual(r.int_s, -52947)
            self.assertEqual(r.div_pos_const, 756)
            self.assertEqual(r.div_neg_const, -757)
            self.assertEqual(r.div_pos_seq, 97130679)
            self.assertEqual(r.div_neg_seq, -4073)
