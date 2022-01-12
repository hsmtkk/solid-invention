package medalrank

import "sort"

type Medal struct {
	Gold   uint
	Silver uint
	Bronze uint
}

func (m Medal) Compare(another Medal) bool {
	if m.Gold == another.Gold {
		if m.Silver == another.Silver {
			if m.Bronze == another.Bronze {
				return false
			} else {
				return m.Bronze > another.Bronze
			}
		} else {
			return m.Silver > another.Silver
		}
	} else {
		return m.Gold > another.Gold
	}
}

func Sort(medals []Medal) []Medal {
	sort.Slice(medals, func(i, j int) bool { return medals[i].Compare(medals[j]) })
	return medals
}
