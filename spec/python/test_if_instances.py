# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from if_instances import IfInstances

class TestIfInstances(unittest.TestCase):
    def test_if_instances(self):
        with IfInstances.from_file('src/fixed_struct.bin') as r:
            self.assertIsNone(r.never_happens)
