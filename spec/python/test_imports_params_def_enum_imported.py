# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from imports_params_def_enum_imported import ImportsParamsDefEnumImported
import enum_0
import enum_deep

class TestImportsParamsDefEnumImported(unittest.TestCase):
    def test_imports_params_def_enum_imported(self):
        with ImportsParamsDefEnumImported.from_file('src/enum_0.bin') as r:

            self.assertEqual(r.one.pet_1, enum_0.Enum0.Animal.cat)
            self.assertEqual(r.one.pet_2, enum_deep.EnumDeep.Container1.Container2.Animal.hare)
            self.assertEqual(r.two.pet_1_param, enum_0.Enum0.Animal.cat)
            self.assertEqual(r.two.pet_2_param, enum_deep.EnumDeep.Container1.Container2.Animal.hare)
