// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::enum_to_i::*;

#[test]
fn test_enum_to_i() {
    let bytes = fs::read("../../src/enum_0.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<EnumToI>> = EnumToI::read_into(&_io, None, None);
    let r : OptRc<EnumToI>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.pet_1(), EnumToI_Animal::Cat);
    assert_eq!(*r.pet_2(), EnumToI_Animal::Chicken);
    assert_eq!(*r.pet_1_i().expect("error reading"), 7);
    assert_eq!(*r.pet_1_mod().expect("error reading"), 32775);
    assert_eq!(*r.one_lt_two().expect("error reading"), true);
    assert_eq!(*r.pet_1_eq_int().expect("error reading"), true);
    assert_eq!(*r.pet_2_eq_int().expect("error reading"), false);
}
