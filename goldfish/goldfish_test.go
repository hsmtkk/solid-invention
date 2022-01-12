package goldfish_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/goldfish"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	p := goldfish.Problem{2, 10, []uint{5, 5, 3, 1, 4}}
	var want uint = 4
	got := p.Solve()
	assert.Equal(t, want, got)
}

func Test1(t *testing.T) {
	p := goldfish.Problem{2, 4, []uint{4, 3, 5, 2, 2}}
	var want uint = 0
	got := p.Solve()
	assert.Equal(t, want, got)
}
