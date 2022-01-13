package pagination_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/pagination"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	p := pagination.Problem{34, 10}
	want := []uint{21, 22, 23, 24, 25, 26, 27, 28, 29, 30}
	got := p.PagesOfIndex(3)
	assert.Equal(t, want, got)
}
