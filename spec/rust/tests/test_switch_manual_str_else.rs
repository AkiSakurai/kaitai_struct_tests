// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::switch_manual_str_else::*;

#[test]
fn test_switch_manual_str_else() {
    let bytes = fs::read("../../src/switch_opcodes2.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<SwitchManualStrElse>> = SwitchManualStrElse::read_into(&_io, None, None);
    let r : OptRc<SwitchManualStrElse>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.opcodes().len(), 4);
    assert_eq!(*r.opcodes()[0 as usize].code(), "S");
    assert_eq!(*Into::<OptRc<SwitchManualStrElse_Opcode_Strval>>::into(&*r.opcodes()[0 as usize].body().as_ref().unwrap()).value(), "foo");
    assert_eq!(*r.opcodes()[1 as usize].code(), "X");
    assert_eq!(*Into::<OptRc<SwitchManualStrElse_Opcode_Noneval>>::into(&*r.opcodes()[1 as usize].body().as_ref().unwrap()).filler(), 66);
    assert_eq!(*r.opcodes()[2 as usize].code(), "Y");
    assert_eq!(*Into::<OptRc<SwitchManualStrElse_Opcode_Noneval>>::into(&*r.opcodes()[2 as usize].body().as_ref().unwrap()).filler(), 51966);
    assert_eq!(*r.opcodes()[3 as usize].code(), "I");
    assert_eq!(*Into::<OptRc<SwitchManualStrElse_Opcode_Intval>>::into(&*r.opcodes()[3 as usize].body().as_ref().unwrap()).value(), 7);
}
