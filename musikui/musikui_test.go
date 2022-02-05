package musikui_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/musikui"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	p := musikui.NewProblem(4, 1)
	want := musikui.NewAnswer(4, 9)
	got, err := p.Solve()
	assert.Nil(t, err)
	assert.Equal(t, want, got)
}

func Test1(t *testing.T) {
	p := musikui.NewProblem(3, 7)
	_, err := p.Solve()
	assert.Error(t, err)
}
