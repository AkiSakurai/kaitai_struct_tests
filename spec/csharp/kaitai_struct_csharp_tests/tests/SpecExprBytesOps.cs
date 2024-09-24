// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExprBytesOps : CommonSpec
    {
        [Test]
        public void TestExprBytesOps()
        {
            var r = ExprBytesOps.FromFile(SourceFile("nav_parent_switch.bin"));

            Assert.AreEqual(r.OneSize, 3);
            Assert.AreEqual(r.OneFirst, 1);
            Assert.AreEqual(r.OneMid, 66);
            Assert.AreEqual(r.OneLast, 255);
            Assert.AreEqual(r.OneLast.ToString(), "255");
            Assert.AreEqual(r.OneMin, 1);
            Assert.AreEqual(r.OneMax, 255);
            Assert.AreEqual(r.OneMax.ToString(), "255");
            Assert.AreEqual(r.OneToS, "\u0001B\u00a0");
            Assert.AreEqual(r.TwoSize, 3);
            Assert.AreEqual(r.TwoFirst, 65);
            Assert.AreEqual(r.TwoMid, 255);
            Assert.AreEqual(r.TwoMid.ToString(), "255");
            Assert.AreEqual(r.TwoLast, 75);
            Assert.AreEqual(r.TwoMin, 65);
            Assert.AreEqual(r.TwoMax, 255);
            Assert.AreEqual(r.TwoMax.ToString(), "255");
            Assert.AreEqual(r.TwoToS, "A\u00a0K");
        }
    }
}
