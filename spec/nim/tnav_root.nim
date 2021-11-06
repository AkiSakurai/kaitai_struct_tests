# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/nav_root
import auxiliary/test_utils

let r = NavRoot.fromFile("src/nav.bin")

assert r.header.qtyEntries == uint32(2)
assert r.header.filenameLen == uint32(8)
assert len(r.index.entries) == int(2)
assert r.index.entries[0].filename == string("FIRST___")
assert r.index.entries[1].filename == string("SECOND__")
