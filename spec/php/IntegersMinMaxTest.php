<?php

namespace Kaitai\Struct\Tests;

class IntegersMinMaxTest extends TestCase {
    public function testIntegersMinMax() {
        $r = IntegersMinMax::fromFile(self::SRC_DIR_PATH . '/integers_min_max.bin');

        $this->assertSame(0, $r->unsignedMin()->u1());
        $this->assertSame(0, $r->unsignedMin()->u2le());
        $this->assertSame(0, $r->unsignedMin()->u4le());
        $this->assertSame(0, $r->unsignedMin()->u8le());
        $this->assertSame(0, $r->unsignedMin()->u2be());
        $this->assertSame(0, $r->unsignedMin()->u4be());
        $this->assertSame(0, $r->unsignedMin()->u8be());
        $this->assertSame(255, $r->unsignedMax()->u1());
        $this->assertSame(65535, $r->unsignedMax()->u2le());
        $this->assertSame(4294967295, $r->unsignedMax()->u4le());
        $this->assertSame(-1, $r->unsignedMax()->u8le()); // signed, not unsigned
        $this->assertSame(65535, $r->unsignedMax()->u2be());
        $this->assertSame(4294967295, $r->unsignedMax()->u4be());
        $this->assertSame(-1, $r->unsignedMax()->u8be()); // signed, not unsigned
        $this->assertSame(-128, $r->signedMin()->s1());
        $this->assertSame(-32768, $r->signedMin()->s2le());
        $this->assertSame(-2147483648, $r->signedMin()->s4le());
        $this->assertSame(intval(-9223372036854775808), $r->signedMin()->s8le()); // PHP bug: PHP_INT_MIN is float (https://bugs.php.net/bug.php?id=76385)
        $this->assertSame(-32768, $r->signedMin()->s2be());
        $this->assertSame(-2147483648, $r->signedMin()->s4be());
        $this->assertSame(intval(-9223372036854775808), $r->signedMin()->s8be()); // PHP bug: PHP_INT_MIN is float (https://bugs.php.net/bug.php?id=76385)
        $this->assertSame(127, $r->signedMax()->s1());
        $this->assertSame(32767, $r->signedMax()->s2le());
        $this->assertSame(2147483647, $r->signedMax()->s4le());
        $this->assertSame(9223372036854775807, $r->signedMax()->s8le());
        $this->assertSame(32767, $r->signedMax()->s2be());
        $this->assertSame(2147483647, $r->signedMax()->s4be());
        $this->assertSame(9223372036854775807, $r->signedMax()->s8be());
    }
}
