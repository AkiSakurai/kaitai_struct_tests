<?php
// Autogenerated from KST: please remove this line if doing any edits by hand!

namespace Kaitai\Struct\Tests;

class EnumIntRangeSTest extends TestCase {
    public function testEnumIntRangeS() {
        $r = EnumIntRangeS::fromFile(self::SRC_DIR_PATH . '/enum_int_range_s.bin');


        $this->assertEquals(\Kaitai\Struct\Tests\EnumIntRangeS\Constants::INT_MIN, $r->f1());
        $this->assertEquals(\Kaitai\Struct\Tests\EnumIntRangeS\Constants::ZERO, $r->f2());
        $this->assertEquals(\Kaitai\Struct\Tests\EnumIntRangeS\Constants::INT_MAX, $r->f3());
    }
}
