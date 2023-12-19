// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::debug_switch_user::*;

#[test]
fn test_debug_switch_user() {
    let bytes = fs::read("../../src/nav_parent_switch.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<DebugSwitchUser>> = DebugSwitchUser::read_into(&_io, None, None);
    let r : OptRc<DebugSwitchUser>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.code(), 1);
    assert_eq!(*Into::<OptRc<DebugSwitchUser_One>>::into(&*r.data().as_ref().unwrap()).val(), -190);
}
