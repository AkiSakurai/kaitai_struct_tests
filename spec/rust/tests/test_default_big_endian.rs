// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::default_big_endian::*;

#[test]
fn test_default_big_endian() {
    let bytes = fs::read("../../src/enum_0.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<DefaultBigEndian>> = DefaultBigEndian::read_into(&_io, None, None);
    let r : OptRc<DefaultBigEndian>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.one(), 117440512);
}
