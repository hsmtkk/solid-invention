package medalrank_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/medalrank"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	input := []medalrank.Medal{
		{3, 5, 9},
		{15, 20, 35},
		{30, 45, 72},
		{15, 20, 31},
		{27, 33, 59},
		{27, 35, 77},
	}
	want := []medalrank.Medal{
		{30, 45, 72},
		{27, 35, 77},
		{27, 33, 59},
		{15, 20, 35},
		{15, 20, 31},
		{3, 5, 9},
	}
	got := medalrank.Sort(input)
	assert.Equal(t, want, got)
}
