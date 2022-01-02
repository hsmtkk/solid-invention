package turukame_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/turukame"
	"github.com/stretchr/testify/assert"
)

type TestPattern struct {
	Problem       turukame.Problem
	ShouldSuccess bool
	Want          turukame.TuruKame
}

func TestTurukame(t *testing.T) {
	patterns := []TestPattern{
		{turukame.Problem{32, 10, 2, 4}, true, turukame.TuruKame{4, 6}},
		{turukame.Problem{52, 65, 18, 76}, false, turukame.TuruKame{}},
		{turukame.Problem{8, 90, 46, 47}, false, turukame.TuruKame{}},
		{turukame.Problem{12, 4, 3, 3}, false, turukame.TuruKame{}},
		{turukame.Problem{2, 2, 1, 1}, true, turukame.TuruKame{1, 1}},
	}
	for _, pattern := range patterns {
		problem := pattern.Problem
		got, err := problem.Solve()
		if pattern.ShouldSuccess {
			assert.Nil(t, err)
			assert.Equal(t, pattern.Want, got)
		} else {
			assert.Error(t, err)
		}
	}
}
