package ticketcounter_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/ticketcounter"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	var want uint = 8
	p := ticketcounter.NewProblem(6, []uint{2, 4, 5})
	got := p.Solve()
	assert.Equal(t, want, got)
}

func Test1(t *testing.T) {
	var want uint = 100
	p := ticketcounter.NewProblem(10, []uint{1, 1, 1, 1, 100})
	got := p.Solve()
	assert.Equal(t, want, got)
}

func Test2(t *testing.T) {
	var want uint = 3
	p := ticketcounter.NewProblem(4, []uint{1, 2})
	got := p.Solve()
	assert.Equal(t, want, got)
}
