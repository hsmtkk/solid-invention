package isvalidip_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/isvalidip"
	"github.com/stretchr/testify/assert"
)

func Test(t *testing.T) {
	patterns := map[string]bool{
		"192.168.0.1":          true,
		"192.400.1.10.1000...": false,
		"4.3.2.1":              true,
		"0..33.444...":         false,
		"1.2.3.4":              true,
	}
	for s, want := range patterns {
		got := isvalidip.IsValidIP(s)
		assert.Equal(t, want, got)
	}
}
