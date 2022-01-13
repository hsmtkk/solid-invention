package festdate_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/festdate"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	from := festdate.NewDate(2000, 12, 10)
	to := festdate.NewDate(2001, 1, 10)
	var want uint = 28
	got := from.After(to)
	assert.Equal(t, want, got)
}

func Test1(t *testing.T) {
	from := festdate.NewDate(1994, 4, 8)
	to := festdate.NewDate(1997, 7, 13)
	var want uint = 591
	got := from.After(to)
	assert.Equal(t, want, got)
}
