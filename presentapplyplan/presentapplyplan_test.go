package presentapplyplan_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/presentapplyplan"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	p := presentapplyplan.Problem{5, 2, 4}
	want := []presentapplyplan.Present{presentapplyplan.N, presentapplyplan.A, presentapplyplan.N, presentapplyplan.AB, presentapplyplan.N}
	got := presentapplyplan.Solve(p)
	assert.Equal(t, want, got)
}

func Test1(t *testing.T) {
	p := presentapplyplan.Problem{6, 3, 2}
	want := []presentapplyplan.Present{presentapplyplan.N, presentapplyplan.B, presentapplyplan.A, presentapplyplan.B, presentapplyplan.N, presentapplyplan.AB}
	got := presentapplyplan.Solve(p)
	assert.Equal(t, want, got)
}
