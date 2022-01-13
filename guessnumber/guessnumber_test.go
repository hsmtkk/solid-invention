package guessnumber_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/guessnumber"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	cs := []*guessnumber.Condition{
		guessnumber.NewCondition(guessnumber.GreaterThan, 30),
		guessnumber.NewCondition(guessnumber.LessThan, 40),
		guessnumber.NewCondition(guessnumber.DividedBy, 5),
	}
	conditions := guessnumber.NewConditions(cs)
	want := 35
	got := conditions.GuessNumber()
	assert.Equal(t, want, got)
}

func Test1(t *testing.T) {
	cs := []*guessnumber.Condition{
		guessnumber.NewCondition(guessnumber.DividedBy, 4),
		guessnumber.NewCondition(guessnumber.LessThan, 90),
		guessnumber.NewCondition(guessnumber.DividedBy, 6),
		guessnumber.NewCondition(guessnumber.GreaterThan, 77),
	}
	conditions := guessnumber.NewConditions(cs)
	want := 84
	got := conditions.GuessNumber()
	assert.Equal(t, want, got)
}
