struct Problem {
    a: u16,
    b: u16,
}

impl Problem{
    fn new(a:u16,b:u16) -> Problem {
        Problem{a, b}
    }

    fn solve(&self) -> Option<Answer> {
        for x in 0..10 {
            for y in 0..10 {
                let left = y * (10 * x + y);
                let right = 100 * self.a + 10 * x + self.b;
                if left == right {
                    return Some(Answer::new(x,y));
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
struct Answer {
    x: u16,
    y: u16,
}

impl Answer{
    fn new(x:u16,y:u16) -> Answer {
        Answer{x, y}
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;
    use super::Answer;

    #[test]
    fn test0(){
        let p = Problem::new(4, 1);
        let want = Answer::new(4, 9);
        let got = p.solve().unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let p = Problem::new(3,7);
        let got = p.solve();
        assert_eq!(None, got);
    }
}