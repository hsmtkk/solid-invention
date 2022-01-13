struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    fn new(year:u32, month:u32, day:u32) -> Date {
        Date{year,month,day}
    }

    fn after(&self, future:Date) -> u32 {
        let mut after = 0;
        let mut my_year = self.year;
        let mut my_month = self.month;
        let mut my_day = self.day;
        loop {
            if my_year == future.year && my_month == future.month && my_day == future.day {
                break;
            }
            after += 1;
            my_day += 1;
            if my_month % 2 == 0 {
                if my_day > 15 {
                    my_month += 1;
                    my_day = 1;
                }
            } else {
                if my_day > 13 {
                    my_month += 1;
                    my_day = 1;
                }
            }
            if my_month > 13 {
                my_year += 1;
                my_month = 1;
            }
        }
        after
    }
}

#[cfg(test)]
mod tests {
    use super::Date;

    #[test]
    fn test0(){
        let from = Date::new(2000, 12, 10);
        let to = Date::new(2001, 1, 10);
        let want = 28;
        let got = from.after(to);
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let from = Date::new(1994, 4, 8);
        let to = Date::new(1997, 7, 13);
        let want = 591;
        let got = from.after(to);
        assert_eq!(want, got);
    }
}
