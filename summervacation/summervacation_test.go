package summervacation_test

import (
	"testing"

	sv "github.com/hsmtkk/solid-invention/summervacation"
	"github.com/stretchr/testify/assert"
)

func Test1(t *testing.T) {
	spans := []sv.Span{
		sv.NewSpan(16, 19),
		sv.NewSpan(14, 17),
	}
	got := sv.NewSolver().Solve(spans)
	assert.True(t, got)
}

func Test2(t *testing.T) {
	spans := []sv.Span{
		sv.NewSpan(22, 23),
		sv.NewSpan(17, 20),
		sv.NewSpan(14, 19),
	}
	got := sv.NewSolver().Solve(spans)
	assert.False(t, got)
}

func Test3(t *testing.T) {
	spans := []sv.Span{
		sv.NewSpan(1, 10),
		sv.NewSpan(2, 9),
		sv.NewSpan(4, 7),
		sv.NewSpan(3, 13),
		sv.NewSpan(5, 6),
		sv.NewSpan(1, 6),
		sv.NewSpan(2, 8),
		sv.NewSpan(4, 5),
		sv.NewSpan(4, 7),
		sv.NewSpan(3, 15),
	}
	got := sv.NewSolver().Solve(spans)
	assert.True(t, got)
}
