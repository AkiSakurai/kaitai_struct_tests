#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_int_div::*;

#[test]
fn test_expr_int_div() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ExprIntDiv>> = ExprIntDiv::read_into(&_io, None, None);
    let r: OptRc<ExprIntDiv>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.int_u(), 1262698832);
    assert_eq!(*r.int_s(), -52947);
    assert_eq!(*r.div_pos_const().expect("error reading"), 756);
    assert_eq!(*r.div_neg_const().expect("error reading"), -756);
    assert_eq!(*r.div_pos_seq().expect("error reading"), 97130679);
    assert_eq!(*r.div_neg_seq().expect("error reading"), -4072);
}
