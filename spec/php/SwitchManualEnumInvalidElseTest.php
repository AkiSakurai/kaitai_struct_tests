<?php
// Autogenerated from KST: please remove this line if doing any edits by hand!

namespace Kaitai\Struct\Tests;

class SwitchManualEnumInvalidElseTest extends TestCase {
    public function testSwitchManualEnumInvalidElse() {
        $r = SwitchManualEnumInvalidElse::fromFile(self::SRC_DIR_PATH . '/enum_negative.bin');

        $this->assertSame(2, count($r->opcodes()));
        $this->assertSame(255, $r->opcodes()[0]->code());
        $this->assertSame(123, $r->opcodes()[0]->body()->value());
        $this->assertSame(1, $r->opcodes()[1]->code());
        $this->assertSame(123, $r->opcodes()[1]->body()->value());
    }
}
