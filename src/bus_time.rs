fn solve(target:u32, times:&[u32]) -> Vec<u32>{
    let mut min_diff = 100;
    for t in times {
        let diff = abs(target, *t);
        if diff < min_diff {
            min_diff = diff;
        }
    }
    let mut results: Vec<u32> = Vec::new();
    for t in times {
        let diff = abs(target, *t);
        if diff == min_diff {
            results.push(*t);
        }
    }
    results.sort();
    results
}

fn abs(a:u32, b:u32) -> u32 {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0(){
        let times = vec![2,1,7,3,10];
        let want = vec![3,7];
        let got = super::solve(5, &times);
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let times = vec![3,1];
        let want = vec![3];
        let got = super::solve(3, &times);
        assert_eq!(want, got);
    }
}