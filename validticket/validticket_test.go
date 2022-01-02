package validticket_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/validticket"
	"github.com/stretchr/testify/assert"
)

func TestGetPatterns(t *testing.T) {
	validator := validticket.NewValidator("paiza")
	want := []string{"p.aiza", "pa.iza", "pai.za", "paiz.a"}
	got := validator.GetPatterns()
	assert.Equal(t, want, got)
}

func Test0(t *testing.T) {
	validator := validticket.NewValidator("paiza")
	patterns := map[string]bool{
		"sdfpaizaoiu":    true,
		"sdfpaizoiu":     false,
		"paxiza":         true,
		"paxizya":        false,
		"ghepaizmakbn":   true,
		"paizzza":        false,
		"abcpadizzzaopq": false,
	}
	for word, want := range patterns {
		got := validator.Validate(word)
		assert.Equal(t, want, got, word)
	}
}

func Test1(t *testing.T) {
	validator := validticket.NewValidator("gbyw")
	patterns := map[string]bool{
		"gbyw":                 true,
		"ggbyw":                true,
		"gbyws":                true,
		"b":                    false,
		"kwflzgltejlcaosivfsy": false,
		"gbyl":                 false,
		"ogbyw":                true,
		"ejyt":                 false,
		"yjlih":                false,
		"bzbrquhbujsxnnoxfsw":  false,
		"ygowvgibywkw":         true,
	}
	for word, want := range patterns {
		got := validator.Validate(word)
		assert.Equal(t, want, got, word)
	}

}
