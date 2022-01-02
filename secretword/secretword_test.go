package secretword_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/secretword"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	board := secretword.NewBoard(`HPPLLM
UROQUV
FBSRZY
DPEFKT
GBBEUY
EMCQFY`)

	want := secretword.NewLocation(1, 2)
	got := board.Find("BEEF")
	assert.Equal(t, want, got)

	want = secretword.NewLocation(1, 0)
	got = board.Find("PORK")
	assert.Equal(t, want, got)
}
