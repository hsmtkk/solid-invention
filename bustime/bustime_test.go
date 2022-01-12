package bustime_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/bustime"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	times := []uint{2, 1, 7, 3, 10}
	want := []uint{3, 7}
	got := bustime.Solve(5, times)
	assert.Equal(t, want, got)
}

func Test1(t *testing.T) {
	times := []uint{3, 1}
	want := []uint{3}
	got := bustime.Solve(3, times)
	assert.Equal(t, want, got)
}
