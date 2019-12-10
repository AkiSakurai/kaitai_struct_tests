// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::EnumLongRangeU;

#[test]
fn test_enum_long_range_u() {
    if let Ok(r) = EnumLongRangeU::from_file("src/enum_long_range_u.bin") {

        assert_eq!(r.f1, EnumLongRangeU__Constants::ZERO);
        assert_eq!(r.f2, EnumLongRangeU__Constants::INT_MAX);
        assert_eq!(r.f3, EnumLongRangeU__Constants::INT_OVER_MAX);
        assert_eq!(r.f4, EnumLongRangeU__Constants::LONG_MAX);
    }
}
