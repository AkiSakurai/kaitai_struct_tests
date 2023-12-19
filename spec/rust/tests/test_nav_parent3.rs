// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::nav_parent3::*;

#[test]
fn test_nav_parent3() {
    let bytes = fs::read("../../src/nav_parent2.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<NavParent3>> = NavParent3::read_into(&_io, None, None);
    let r : OptRc<NavParent3>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.ofs_tags(), 8);
    assert_eq!(*r.num_tags(), 2);
    assert_eq!(*r.tags().expect("error reading")[0 as usize].name(), "RAHC");
    assert_eq!(*r.tags().expect("error reading")[0 as usize].ofs(), 32);
    assert_eq!(*r.tags().expect("error reading")[0 as usize].num_items(), 3);
    assert_eq!(*r.tags().expect("error reading")[0 as usize].tag_content().expect("error reading").as_ref().unwrap().content(), "foo");
    assert_eq!(*r.tags().expect("error reading")[1 as usize].name(), "RAHC");
    assert_eq!(*r.tags().expect("error reading")[1 as usize].ofs(), 35);
    assert_eq!(*r.tags().expect("error reading")[1 as usize].num_items(), 6);
    assert_eq!(*r.tags().expect("error reading")[1 as usize].tag_content().expect("error reading").as_ref().unwrap().content(), "barbaz");
}
