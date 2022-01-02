package validticket

import (
	"regexp"
	"strings"
)

type Validator struct {
	original string
}

func NewValidator(original string) *Validator {
	return &Validator{original}
}

func (v *Validator) Validate(word string) bool {
	if strings.Contains(word, v.original) {
		return true
	}
	for _, pattern := range v.GetPatterns() {
		reg := regexp.MustCompile(pattern)
		matched := reg.FindString(word)
		if matched != "" {
			return true
		}
	}
	return false
}

func (v *Validator) GetPatterns() []string {
	patterns := []string{}
	for i := 1; i < len(v.original); i++ {
		first := v.original[:i]
		second := v.original[i:]
		pattern := first + "." + second
		patterns = append(patterns, pattern)
	}
	return patterns
}
