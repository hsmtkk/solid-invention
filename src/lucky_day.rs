struct RuckyDaysGetter {}

impl RuckyDaysGetter {
    fn new() -> RuckyDaysGetter {
        RuckyDaysGetter {}
    }

    fn get_rucky_days(&self, target: u32) -> u32 {
        let mut rucky_days = 0;
        for i in 1..365 {
            if self.is_rucky_day(target, i) {
                rucky_days += 1;
            }
        }
        rucky_days
    }

    fn is_rucky_day(&self, target: u32, i: u32) -> bool {
        let ts = target.to_string();
        let is = i.to_string();
        is.contains(&ts)
    }
}

#[cfg(test)]
mod tests {
    use super::RuckyDaysGetter;

    #[test]
    fn test0() {
        let g = RuckyDaysGetter::new();
        assert_eq!(14, g.get_rucky_days(15));
    }

    #[test]
    fn test1() {
        let g = RuckyDaysGetter::new();
        assert_eq!(1, g.get_rucky_days(128));
    }

    #[test]
    fn test2() {
        let g = RuckyDaysGetter::new();
        assert_eq!(68, g.get_rucky_days(6));
    }
}
