# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from repeat_n_struct import RepeatNStruct

class TestRepeatNStruct(unittest.TestCase):
    def test_repeat_n_struct(self):
        with RepeatNStruct.from_file('src/repeat_n_struct.bin') as r:
            self.assertEqual(len(r.chunks), 2)
            self.assertEqual(r.chunks[0].offset, 16)
            self.assertEqual(r.chunks[0].len, 8312)
            self.assertEqual(r.chunks[1].offset, 8328)
            self.assertEqual(r.chunks[1].len, 15)
