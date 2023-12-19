// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::valid_fail_expr::*;

#[test]
fn test_valid_fail_expr() {
    let bytes = fs::read("../../src/nav_parent_switch.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ValidFailExpr>> = ValidFailExpr::read_into(&_io, None, None);
    let r : OptRc<ValidFailExpr>;

    if let Err(err) = res {
        println!("expected err: {:?}, exception: ValidationExprError(IntMultiType(true,Width2,None))", err);
    } else {
        panic!("no expected exception: ValidationExprError(IntMultiType(true,Width2,None))");
    }
}
