struct Problem {
    customers: u32,
    process_times: Vec<u32>,
    counters: Vec<Option<u32>>,
}

impl Problem {
    fn new(customers:u32, process_times:Vec<u32>) ->Problem{
        let mut counters = Vec::new();
        for _p in &process_times {
            counters.push(None);
        }
        Problem{customers, process_times, counters}
    }

    fn solve(&mut self) -> u32 {
        let mut time = 0;
        let mut customers = self.customers;
        loop {
            for i in 0..self.counters.len(){
                if customers > 0 && self.counters[i] == None {
                    self.counters[i] = Some(0);
                    customers -= 1;
                }
            }
            if customers == 0 && self.all_empty(){
                break;
            }
            for i in 0..self.counters.len(){
                if let Some(n) = self.counters[i]{
                    self.counters[i] = Some(n+1);
                }
            }
            for i in 0..self.counters.len(){
                if let Some(n) = self.counters[i]{
                    if n >= self.process_times[i]{
                        self.counters[i] = None;
                    }
                }
            }
            time += 1;
        }
        time
    }

    fn all_empty(&self) -> bool {
        for c in &self.counters {
            if *c != None {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;

    #[test]
    fn test0(){
        let process_times = vec![2, 4, 5];
        let mut problem = Problem::new(6, process_times);
        let got = problem.solve();
        let want = 8;
        assert_eq!(want,got);
    }

    #[test]
    fn test2(){
        let process_times = vec![1,1,1,1,100];
        let mut problem = Problem::new(5, process_times);
        let got = problem.solve();
        let want = 100;
        assert_eq!(want,got);
    }

    #[test]
    fn test3(){
        let process_times = vec![1,2];
        let mut problem = Problem::new(4, process_times);
        let got = problem.solve();
        let want = 3;
        assert_eq!(want,got);
    }
}