// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::nav_parent_false::*;

#[test]
fn test_nav_parent_false() {
    let bytes = fs::read("../../src/nav_parent_codes.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<NavParentFalse>> = NavParentFalse::read_into(&_io, None, None);
    let r : OptRc<NavParentFalse>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.child_size(), 3);
    assert_eq!(*r.element_a().foo().code(), 73);
    assert_eq!(*r.element_a().foo().more(), vec![0x31u8, 0x32u8, 0x33u8]);
    assert_eq!(*r.element_a().bar().foo().code(), 66);
    assert_eq!(*r.element_b().foo().code(), 98);
}
