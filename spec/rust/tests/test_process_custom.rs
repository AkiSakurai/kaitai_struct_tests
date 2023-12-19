// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::process_custom::*;

#[test]
fn test_process_custom() {
    let bytes = fs::read("../../src/process_rotate.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ProcessCustom>> = ProcessCustom::read_into(&_io, None, None);
    let r : OptRc<ProcessCustom>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.buf1(), vec![0x10u8, 0xb3u8, 0x94u8, 0x94u8, 0xf4u8]);
    assert_eq!(*r.buf2(), vec![0x5fu8, 0xbau8, 0x7bu8, 0x93u8, 0x63u8, 0x23u8, 0x5fu8]);
    assert_eq!(*r.buf3(), vec![0x29u8, 0x33u8, 0xb1u8, 0x38u8, 0xb1u8]);
}
