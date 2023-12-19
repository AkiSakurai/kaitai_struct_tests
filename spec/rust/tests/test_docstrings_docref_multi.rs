// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::docstrings_docref_multi::*;

#[test]
fn test_docstrings_docref_multi() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<DocstringsDocrefMulti>> = DocstringsDocrefMulti::read_into(&_io, None, None);
    let r : OptRc<DocstringsDocrefMulti>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

}
