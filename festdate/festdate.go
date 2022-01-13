package festdate

type Date struct {
	year  uint
	month uint
	day   uint
}

func NewDate(year, month, day uint) Date {
	return Date{year, month, day}
}

func (d Date) After(future Date) uint {
	var after uint = 0
	for {
		if d.year == future.year && d.month == future.month && d.day == future.day {
			break
		}
		after += 1
		d.day += 1
		if d.month%2 == 0 {
			if d.day >= 16 {
				d.month += 1
				d.day = 1
			}
		} else {
			if d.day >= 14 {
				d.month += 1
				d.day = 1
			}
		}
		if d.month >= 14 {
			d.year += 1
			d.month = 1
		}
	}
	return after
}
