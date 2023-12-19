// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_sizeof_value_0::*;

#[test]
fn test_expr_sizeof_value_0() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ExprSizeofValue0>> = ExprSizeofValue0::read_into(&_io, None, None);
    let r : OptRc<ExprSizeofValue0>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.self_sizeof().expect("error reading"), (((((1 + 4) as i32) + (2 as i32)) as i32) + (2 as i32)));
    assert_eq!(*r.sizeof_block().expect("error reading"), (((1 + 4) as i32) + (2 as i32)));
    assert_eq!(*r.sizeof_block_a().expect("error reading"), 1);
    assert_eq!(*r.sizeof_block_b().expect("error reading"), 4);
    assert_eq!(*r.sizeof_block_c().expect("error reading"), 2);
}
