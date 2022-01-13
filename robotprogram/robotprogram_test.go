package robotprogram_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/robotprogram"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	requests := []robotprogram.Request{
		{2, "foo"},
		{3, "bar"},
	}
	want := []robotprogram.Response{
		robotprogram.NewResponseNumber(1),
		robotprogram.NewResponseActions([]string{"foo"}),
		robotprogram.NewResponseActions([]string{"bar"}),
		robotprogram.NewResponseActions([]string{"foo"}),
		robotprogram.NewResponseNumber(5),
		robotprogram.NewResponseActions([]string{"foo", "bar"}),
	}
	got := robotprogram.Process(6, requests)
	assert.Equal(t, want, got)
}
