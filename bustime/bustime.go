package bustime

import (
	"sort"
)

func Solve(target uint, times []uint) []uint {
	var minDiff uint = 100
	for _, t := range times {
		diff := abs(t, target)
		if diff < minDiff {
			minDiff = diff
		}
	}
	results := []uint{}
	for _, t := range times {
		diff := abs(t, target)
		if diff == minDiff {
			results = append(results, t)
		}
	}
	sort.Slice(results, func(i, j int) bool { return results[i] < results[j] })
	return results
}

func abs(a, b uint) uint {
	if a >= b {
		return a - b
	} else {
		return b - a
	}
}
