<?php
// Autogenerated from KST: please remove this line if doing any edits by hand!

namespace Kaitai\Struct\Tests;

class NavParent2Test extends TestCase {
    public function testNavParent2() {
        $r = NavParent2::fromFile(self::SRC_DIR_PATH . '/nav_parent2.bin');

        $this->assertSame(8, $r->ofsTags());
        $this->assertSame(2, $r->numTags());
        $this->assertSame("RAHC", $r->tags()[0]->name());
        $this->assertSame(32, $r->tags()[0]->ofs());
        $this->assertSame(3, $r->tags()[0]->numItems());
        $this->assertSame("foo", $r->tags()[0]->tagContent()->content());
        $this->assertSame("RAHC", $r->tags()[1]->name());
        $this->assertSame(35, $r->tags()[1]->ofs());
        $this->assertSame(6, $r->tags()[1]->numItems());
        $this->assertSame("barbaz", $r->tags()[1]->tagContent()->content());
    }
}
