package origamipaste_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/origamipaste"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	p := &origamipaste.Problem{
		Count:    3,
		Edge:     4,
		Overlaps: []uint32{2, 1},
	}
	assert.Equal(t, uint32(36), p.Solve())
}

func Test1(t *testing.T) {
	p := &origamipaste.Problem{
		Count:    4,
		Edge:     10,
		Overlaps: []uint32{3, 4, 5},
	}
	assert.Equal(t, uint32(280), p.Solve())
}
