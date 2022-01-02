use anyhow::{bail, Result};

struct Problem {
    leg_sum: u32,
    head_sum: u32,
    turu_leg: u32,
    kame_leg: u32,
}

impl Problem {
    fn new(leg_sum:u32, head_sum:u32, turu_leg:u32, kame_leg:u32) -> Problem {
        Problem{leg_sum, head_sum, turu_leg, kame_leg}
    }

    fn solve(&self) -> Result<(u32, u32)> {
        let mut found = Vec::new();
        for turu in 1..self.head_sum {
            let kame = self.head_sum  - turu;
            let legs = self.turu_leg * turu + self.kame_leg * kame;
            if legs == self.leg_sum {
                found.push((turu, kame));
            }
        }
        if found.len() == 1 {
            Ok(found[0])
        } else {
            bail!("miss")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;

    #[test]
    fn test0(){
        let problem = Problem::new(32,10,2,4);
        let want = (4,6);
        let got = problem.solve().unwrap();
        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn test1(){
        let problem = Problem::new(52, 65, 18, 76);
        problem.solve().unwrap();
    }

    #[test]
    #[should_panic]
    fn test2(){
        let problem = Problem::new(8, 90, 46, 47);
        problem.solve().unwrap();
    }

    #[test]
    #[should_panic]
    fn test3(){
        let problem = Problem::new(12, 4, 3, 3);
        let got = problem.solve().unwrap();
    }

    #[test]
    fn test4(){
        let problem = Problem::new(2, 2, 1, 1);
        let want = (1,1);
        let got = problem.solve().unwrap();
        assert_eq!(want, got);
    }
}