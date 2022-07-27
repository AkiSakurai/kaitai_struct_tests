#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::valid_fail_range_bytes::*;

#[test]
fn test_valid_fail_range_bytes() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ValidFailRangeBytes::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {
        println!("expected err: {:?}, exception: ValidationGreaterThanError(CalcBytesType)", err);
    } else {
        panic!("no expected exception: ValidationGreaterThanError(CalcBytesType)");
    }
}
