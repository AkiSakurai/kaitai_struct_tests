// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_if_int_ops::*;

#[test]
fn test_expr_if_int_ops() {
    let bytes = fs::read("../../src/process_coerce_switch.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ExprIfIntOps>> = ExprIfIntOps::read_into(&_io, None, None);
    let r : OptRc<ExprIfIntOps>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.is_eq_prim().expect("error reading"), true);
    assert_eq!(*r.is_eq_boxed().expect("error reading"), true);
}
