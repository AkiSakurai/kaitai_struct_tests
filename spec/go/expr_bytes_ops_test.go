package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
	"strconv"
)

func TestExprBytesOps(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/nav_parent_switch.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r ExprBytesOps
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	tmp1, err := r.OneSize()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 3, tmp1)
	tmp2, err := r.OneFirst()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 1, tmp2)
	tmp3, err := r.OneMid()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 66, tmp3)
	tmp4, err := r.OneLast()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 255, tmp4)
	tmp5, err := r.OneLast()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "255", strconv.FormatInt(int64(tmp5), 10))
	tmp6, err := r.OneMin()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 1, tmp6)
	tmp7, err := r.OneMax()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 255, tmp7)
	tmp8, err := r.OneMax()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "255", strconv.FormatInt(int64(tmp8), 10))
	tmp9, err := r.OneToS()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "\001B\240", tmp9)
	tmp10, err := r.TwoSize()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 3, tmp10)
	tmp11, err := r.TwoFirst()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 65, tmp11)
	tmp12, err := r.TwoMid()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 255, tmp12)
	tmp13, err := r.TwoMid()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "255", strconv.FormatInt(int64(tmp13), 10))
	tmp14, err := r.TwoLast()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 75, tmp14)
	tmp15, err := r.TwoMin()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 65, tmp15)
	tmp16, err := r.TwoMax()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 255, tmp16)
	tmp17, err := r.TwoMax()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "255", strconv.FormatInt(int64(tmp17), 10))
	tmp18, err := r.TwoToS()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "A\240K", tmp18)
}
