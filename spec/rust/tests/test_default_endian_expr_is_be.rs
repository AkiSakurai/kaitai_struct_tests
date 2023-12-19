// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::default_endian_expr_is_be::*;

#[test]
fn test_default_endian_expr_is_be() {
    let bytes = fs::read("../../src/endian_expr.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<DefaultEndianExprIsBe>> = DefaultEndianExprIsBe::read_into(&_io, None, None);
    let r : OptRc<DefaultEndianExprIsBe>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.docs()[0 as usize].indicator(), vec![0x49u8, 0x49u8]);
    assert_eq!(*r.docs()[0 as usize].main().some_int(), 66);
    assert_eq!(*r.docs()[0 as usize].main().some_int_be(), 66);
    assert_eq!(*r.docs()[0 as usize].main().some_int_le(), 66);
    assert_eq!(*r.docs()[0 as usize].main().inst_int().expect("error reading"), 66);
    assert_eq!(*r.docs()[0 as usize].main().inst_sub().expect("error reading").foo(), 66);
    assert_eq!(*r.docs()[1 as usize].indicator(), vec![0x4du8, 0x4du8]);
    assert_eq!(*r.docs()[1 as usize].main().some_int(), 66);
    assert_eq!(*r.docs()[1 as usize].main().some_int_be(), 66);
    assert_eq!(*r.docs()[1 as usize].main().some_int_le(), 66);
    assert_eq!(*r.docs()[1 as usize].main().inst_int().expect("error reading"), 1107296256);
    assert_eq!(*r.docs()[1 as usize].main().inst_sub().expect("error reading").foo(), 1107296256);
    assert_eq!(*r.docs()[2 as usize].indicator(), vec![0x58u8, 0x58u8]);
    assert_eq!(*r.docs()[2 as usize].main().some_int(), 1107296256);
    assert_eq!(*r.docs()[2 as usize].main().some_int_be(), 66);
    assert_eq!(*r.docs()[2 as usize].main().some_int_le(), 66);
    assert_eq!(*r.docs()[2 as usize].main().inst_int().expect("error reading"), 66);
    assert_eq!(*r.docs()[2 as usize].main().inst_sub().expect("error reading").foo(), 66);
}
