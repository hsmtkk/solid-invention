package aikotoba_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/aikotoba"
	"github.com/stretchr/testify/assert"
)

func Test(t *testing.T) {
	auth := aikotoba.New("abc")
	assert.True(t, auth.Authenticate("bac"))
	assert.False(t, auth.Authenticate("abc"))
	assert.False(t, auth.Authenticate("xy"))
}
