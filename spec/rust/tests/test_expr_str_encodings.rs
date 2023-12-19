// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_str_encodings::*;

#[test]
fn test_expr_str_encodings() {
    let bytes = fs::read("../../src/str_encodings.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ExprStrEncodings>> = ExprStrEncodings::read_into(&_io, None, None);
    let r : OptRc<ExprStrEncodings>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.str1_eq().expect("error reading"), true);
    assert_eq!(*r.str2_eq().expect("error reading"), true);
    assert_eq!(*r.str3_eq().expect("error reading"), true);
    assert_eq!(*r.str3_eq_str2().expect("error reading"), true);
    assert_eq!(*r.str4_eq().expect("error reading"), true);
    assert_eq!(*r.str4_gt_str_calc().expect("error reading"), true);
    assert_eq!(*r.str4_gt_str_from_bytes().expect("error reading"), true);
}
