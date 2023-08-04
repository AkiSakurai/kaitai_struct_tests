// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprToITrailing;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
import io.kaitai.struct.KaitaiStream;
public class TestExprToITrailing extends CommonSpec {

    @Test
    public void testExprToITrailing() throws Exception {
        ExprToITrailing r = ExprToITrailing.fromFile(SRC_DIR + "term_strz.bin");

        assertThrows(NumberFormatException.class, () -> r.toIR10());
        assertIntEquals(r.toIR16(), 152517308);
        assertThrows(NumberFormatException.class, () -> r.toIGarbage());
    }
}
