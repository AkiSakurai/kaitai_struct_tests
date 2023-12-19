// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::nested_types::*;

#[test]
fn test_nested_types() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<NestedTypes>> = NestedTypes::read_into(&_io, None, None);
    let r : OptRc<NestedTypes>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.one().typed_at_root().value_b(), 80);
    assert_eq!(*r.one().typed_here().value_c(), 65);
    assert_eq!(*r.two().value_b(), 67);
}
