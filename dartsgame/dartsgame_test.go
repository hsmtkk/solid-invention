package dartsgame_test

import (
	"math"
	"testing"

	"github.com/hsmtkk/solid-invention/dartsgame"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	darts := dartsgame.Darts{10, 10, 10}
	target := dartsgame.Target{10, 10, 10}
	game := dartsgame.Game{darts, target}
	got, err := game.Solve()
	want := 3.3
	diff := math.Abs(want - got)
	assert.Nil(t, err)
	assert.True(t, diff < 0.1)
}

func Test1(t *testing.T) {
	darts := dartsgame.Darts{10, 15, 45}
	target := dartsgame.Target{10, 10, 10}
	game := dartsgame.Game{darts, target}
	_, err := game.Solve()
	assert.Error(t, err)
}
