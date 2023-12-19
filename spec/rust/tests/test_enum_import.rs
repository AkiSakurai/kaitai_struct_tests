// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::enum_import::*;
use rust::formats::enum_0::*;
use rust::formats::enum_deep::*;

#[test]
fn test_enum_import() {
    let bytes = fs::read("../../src/enum_0.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<EnumImport>> = EnumImport::read_into(&_io, None, None);
    let r : OptRc<EnumImport>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.pet_1(), Enum0_Animal::Cat);
    assert_eq!(*r.pet_2(), EnumDeep_Container1_Container2_Animal::Hare);
}
