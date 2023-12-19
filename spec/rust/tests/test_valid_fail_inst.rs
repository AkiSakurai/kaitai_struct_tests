// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::valid_fail_inst::*;

#[test]
fn test_valid_fail_inst() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ValidFailInst>> = ValidFailInst::read_into(&_io, None, None);
    let r : OptRc<ValidFailInst>;

    if let Err(err) = res {
        println!("expected err: {:?}, exception: ValidationNotEqualError(Int1Type(false))", err);
    } else {
        panic!("no expected exception: ValidationNotEqualError(Int1Type(false))");
    }
}
