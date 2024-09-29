# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from expr_io_ternary import ExprIoTernary

class TestExprIoTernary(unittest.TestCase):
    def test_expr_io_ternary(self):
        with ExprIoTernary.from_file('src/if_struct.bin') as r:
            self.assertEqual(r.one_or_two_io_size1, 8)
            self.assertEqual(r.one_or_two_io_size2, 8)
            self.assertEqual(r.one_or_two_io_size_add_3, 11)
