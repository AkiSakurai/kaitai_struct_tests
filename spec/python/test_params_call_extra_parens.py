# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from params_call_extra_parens import ParamsCallExtraParens

class TestParamsCallExtraParens(unittest.TestCase):
    def test_params_call_extra_parens(self):
        with ParamsCallExtraParens.from_file('src/term_strz.bin') as r:
            self.assertEqual(r.buf1.body, u"foo|b")
