type Goods = Vec<u32>;

struct Problem {
    duration: usize,
    threshold: u32,
    goods: Goods,
}

impl Problem {
    fn new(duration:usize, threshold:u32, goods:Goods) -> Problem {
        Problem{duration, threshold, goods}
    }

    fn solve(&self) -> Option<u32> {
        let mut buzzed: u32 = 0;
        for i in 0..self.goods.len(){
            let j = i + self.duration;
            if j > self.goods.len(){
                break;
            }
            if sum_array(&self.goods[i..j]) >= self.threshold {
                buzzed = (j-1).try_into().unwrap();
            }
        }
        if buzzed != 0 {
            Some(buzzed)
        } else {
            None
        }
    }
}

pub fn sum_array(ns: &[u32]) -> u32{
    let mut s = 0;
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
        let problem = Problem::new(3, 1000, vec![100, 500, 60, 100, 10]);
        assert_eq!(None, problem.solve());

        let problem = Problem::new(3,1000, vec![100, 200,300,500,1000]);
        assert_eq!(Some(4), problem.solve());
    }

    #[test]
    fn test1(){
        let problem = Problem::new(4, 12, vec![1,1,1,1]);
        assert_eq!(None, problem.solve());

        let problem = Problem::new(4, 12, vec![1,2,3,4]);
        assert_eq!(None, problem.solve());

        let problem = Problem::new(4, 12, vec![1,3,6,10]);
        assert_eq!(Some(4), problem.solve());

        let problem = Problem::new(4, 12, vec![1,4,10,20]);
        assert_eq!(Some(3), problem.solve());
    }
}