# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/process_custom
import auxiliary/test_utils

let r = ProcessCustom.fromFile("src/process_rotate.bin")

assert r.buf1 == @[16'u8, -77'u8, -108'u8, -108'u8, -12'u8]
assert r.buf2 == @[95'u8, -70'u8, 123'u8, -109'u8, 99'u8, 35'u8, 95'u8]
assert r.buf3 == @[41'u8, 51'u8, -79'u8, 56'u8, -79'u8]
