use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::repeat_n_struct::*;


#[test]
fn test_repeat_n_struct() {
    let bytes = fs::read("../../src/repeat_n_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = RepeatNStruct::read_into(&_io, None, None);
    let r : Rc<RepeatNStruct>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.chunks().len(), 2);
    assert_eq!(*r.chunks()[0 as usize].offset(), 16);
    assert_eq!(*r.chunks()[0 as usize].len(), 8312);
    assert_eq!(*r.chunks()[1 as usize].offset(), 8328);
    assert_eq!(*r.chunks()[1 as usize].len(), 15);
}
