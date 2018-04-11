// Autogenerated from KST: please remove this line if doing any edits by hand!

package spec

import (
	"os"
	"testing"
	"github.com/stretchr/testify/assert"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
)

func TestProcessXor4Const(t *testing.T) {
	f, err := os.Open("../../src/process_xor_4.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r ProcessXor4Const
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, []uint8{236, 187, 163, 20}, r.Key)
	assert.EqualValues(t, []uint8{102, 111, 111, 32, 98, 97, 114}, r.Buf)
}
