# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/str_eos
import auxiliary/test_utils

let r = StrEos.fromFile("src/term_strz.bin")

assert r.str == string("foo|bar|baz@")
