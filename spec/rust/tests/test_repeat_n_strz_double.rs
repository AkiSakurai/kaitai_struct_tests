// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::repeat_n_strz_double::*;

#[test]
fn test_repeat_n_strz_double() {
    let bytes = fs::read("../../src/repeat_n_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<RepeatNStrzDouble>> = RepeatNStrzDouble::read_into(&_io, None, None);
    let r : OptRc<RepeatNStrzDouble>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.qty(), 2);
    assert_eq!(*r.lines1(), vec!["foo".to_string()]);
    assert_eq!(*r.lines2(), vec!["bar".to_string()]);
}
