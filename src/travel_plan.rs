struct Forecast {
    day: u32,
    rain_percent: u32,
}

impl Forecast {
    fn new(day:u32, rain_percent:u32) -> Forecast{
        Forecast{day, rain_percent}
    }
}

#[derive(Debug, PartialEq)]
struct Plan {
    begin: u32,
    end: u32,
}

impl Plan {
    fn new(begin:u32, end:u32) -> Plan {
        Plan{begin, end}
    }
}

fn create_plan(forecasts:&[Forecast], days:u32) -> Plan {
    let mut min_percent = 100.0;
    let mut plan = Plan::new(0, 0);
    for from in 0..forecasts.len()-(days as usize) {
        let avg_percent = calculate_average_percent(forecasts, days, from);
        if avg_percent < min_percent {
            min_percent = avg_percent;
            let begin: u32 = forecasts[from].day;
            plan = Plan::new(begin, begin + days - 1);
        }
    }
    plan
}

fn calculate_average_percent(forecasts:&[Forecast], days:u32, from:usize) -> f32 {
    let mut s = 0;
    for i in from..(from+days as usize) {
        let f = &forecasts[i];
        s += f.rain_percent;
    }
    let avg = s as f32 / days as f32;
    avg
}

#[cfg(test)]
mod tests {
    use super::{Forecast, Plan};

    #[test]
    fn test0(){
        let forecasts = vec![
            Forecast::new(19, 0),
            Forecast::new(20, 0),
            Forecast::new(21, 60),
            Forecast::new(22, 30),
            Forecast::new(23, 10),
            Forecast::new(24, 10),
            Forecast::new(25, 90),
        ];
        let want = Plan::new(22, 24);
        let got = super::create_plan(&forecasts, 3);
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let forecasts = vec![
            Forecast::new(3, 30),
            Forecast::new(4, 25),
            Forecast::new(5, 20),
            Forecast::new(6, 65),
            Forecast::new(7, 75),
            Forecast::new(8, 0),
            Forecast::new(9, 10),
        ];
        let want = Plan::new(3, 6);
        let got = super::create_plan(&forecasts, 4);
        assert_eq!(want, got);
    }
}