struct Problem {
    count: u32,
    edge: u32,
    overlaps: Vec<u32>,
}

impl Problem {
    fn new(count:u32, edge:u32, overlaps:Vec<u32>) -> Problem {
        Problem{count, edge, overlaps}
    }

    fn solve(&self) -> u32 {
        self.edge * (self.count * self.edge - (sum(&self.overlaps)))
    }
}

fn sum(ns:&[u32]) -> u32 {
    let mut s= 0;
    for n in ns {
        s += n;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::Problem;

    #[test]
    fn test0(){
        let p = Problem::new(3, 4, vec![2,1]);
        assert_eq!(36, p.solve());
    }

    #[test]
    fn test1(){
        let p = Problem::new(4, 10, vec![3,4,5]);
        assert_eq!(280, p.solve());
    }
}