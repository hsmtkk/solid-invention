package ruckydays

import (
	"strconv"
	"strings"
)

type ruckyDaysGetter struct{}

func New() *ruckyDaysGetter {
	return &ruckyDaysGetter{}
}

func (g *ruckyDaysGetter) GetRuckyDays(target int) int {
	luckyDays := 0
	for i := 1; i < 365; i++ {
		if g.isRuckyDay(target, i) {
			luckyDays += 1
		}
	}
	return luckyDays
}

func (g *ruckyDaysGetter) isRuckyDay(target, i int) bool {
	ts := strconv.Itoa(target)
	is := strconv.Itoa(i)
	return strings.Contains(is, ts)
}
