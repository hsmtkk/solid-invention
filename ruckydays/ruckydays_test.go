package ruckydays_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/ruckydays"
	"github.com/stretchr/testify/assert"
)

type RuckyDaysGetter interface {
	GetRuckyDays(target int) int
}

func Test0(t *testing.T) {
	var getter RuckyDaysGetter = ruckydays.New()
	assert.Equal(t, 14, getter.GetRuckyDays(15))
}

func Test1(t *testing.T) {
	var getter RuckyDaysGetter = ruckydays.New()
	assert.Equal(t, 1, getter.GetRuckyDays(128))
}

func Test2(t *testing.T) {
	var getter RuckyDaysGetter = ruckydays.New()
	assert.Equal(t, 68, getter.GetRuckyDays(6))
}
