#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_int_size::*;

#[test]
fn test_switch_manual_int_size() {
    let bytes = fs::read("../../src/switch_tlv.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = SwitchManualIntSize::read_into(&_io, None, None);
    let r : Rc<SwitchManualIntSize>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }
    assert_eq!(4, r.chunks().len());

    assert_eq!(17, *r.chunks()[0].code());
    if let Some(SwitchManualIntSize_Chunk_Body::SwitchManualIntSize_Chunk_ChunkMeta(s)) =  r.chunks()[0].body().as_ref() {
        assert_eq!("Stuff", *s.title());
        assert_eq!("Me", *s.author());
    } else {
        panic!("expected enum SwitchManualIntSize_Chunk_ChunkMeta");
    }

    assert_eq!(34, *r.chunks()[1].code());
    if let Some(SwitchManualIntSize_Chunk_Body::SwitchManualIntSize_Chunk_ChunkDir(s)) =  r.chunks()[1].body().as_ref() {
        let strings : Vec<String> = vec!["AAAA", "BBBB", "CCCC"].iter().map(|&s| s.to_string() ).collect();
        assert_eq!(strings, *s.entries());
    } else {
        panic!("expected enum SwitchManualIntSize_Chunk_ChunkDir");
    }
 
    assert_eq!(51, *r.chunks()[2].code());
    if let Some(SwitchManualIntSize_Chunk_Body::Bytes(s)) =  r.chunks()[2].body().as_ref() {
        let raw : Vec<u8> = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    }
 
    assert_eq!(255, *r.chunks()[3].code());
    if let Some(SwitchManualIntSize_Chunk_Body::Bytes(s)) =  r.chunks()[3].body().as_ref() {
        let raw : Vec<u8> = vec![];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    };
}
