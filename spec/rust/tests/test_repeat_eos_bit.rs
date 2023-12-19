// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::repeat_eos_bit::*;

#[test]
fn test_repeat_eos_bit() {
    let bytes = fs::read("../../src/enum_0.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<RepeatEosBit>> = RepeatEosBit::read_into(&_io, None, None);
    let r : OptRc<RepeatEosBit>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.nibbles().len(), 16);
}
