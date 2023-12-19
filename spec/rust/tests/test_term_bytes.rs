// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::term_bytes::*;

#[test]
fn test_term_bytes() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<TermBytes>> = TermBytes::read_into(&_io, None, None);
    let r : OptRc<TermBytes>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.s1(), vec![0x66u8, 0x6fu8, 0x6fu8]);
    assert_eq!(*r.s2(), vec![0x62u8, 0x61u8, 0x72u8]);
    assert_eq!(*r.s3(), vec![0x7cu8, 0x62u8, 0x61u8, 0x7au8, 0x40u8]);
}
