// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_array::*;

#[test]
fn test_expr_array() {
    let bytes = fs::read("../../src/expr_array.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ExprArray>> = ExprArray::read_into(&_io, None, None);
    let r : OptRc<ExprArray>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.aint_size().expect("error reading"), 4);
    assert_eq!(*r.aint_first().expect("error reading"), 7657765);
    assert_eq!(*r.aint_last().expect("error reading"), 16272640);
    assert_eq!(*r.aint_min().expect("error reading"), 49185);
    assert_eq!(*r.aint_max().expect("error reading"), 1123362332);
    assert_eq!(*r.afloat_size().expect("error reading"), 3);
    assert_eq!(*r.afloat_first().expect("error reading"), -2.6839530254859364E-121);
    assert_eq!(*r.afloat_last().expect("error reading"), -1.1103359815095273E-175);
    assert_eq!(*r.afloat_min().expect("error reading"), -8.754689149998834E+288);
    assert_eq!(*r.afloat_max().expect("error reading"), -1.1103359815095273E-175);
    assert_eq!(*r.astr_size().expect("error reading"), 3);
    assert_eq!(*r.astr_first().expect("error reading"), "foo");
    assert_eq!(*r.astr_last().expect("error reading"), "baz");
    assert_eq!(*r.astr_min().expect("error reading"), "bar");
    assert_eq!(*r.astr_max().expect("error reading"), "foo");
}
