use std::collections::HashMap;

fn most_used(input: &[u32]) -> Vec<u32> {
    let mut counter: HashMap<u32, u32> = HashMap::new();
    for n in input {
        match counter.get(&*n) {
            Some(c) => {
                counter.insert(*n, c+1);
            },
            None => {
                counter.insert(*n, 1);
            },
        }
    }
    let mut counts: Vec<u32> = counter.values().cloned().collect();
    counts.sort();
    counts.reverse();
    let max_count = counts[0];
    let mut results = Vec::new();
    for (n, c) in counter {
        if c == max_count {
            results.push(n);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0(){
        let input = vec![1, 1, 2, 2, 3];
        let want = vec![1,2];
        let got = super::most_used(&input);
        assert!(equal_vector(&want, &got));
    }

    #[test]
    fn test1(){
        let input = vec![1, 2, 3, 1 ,1];
        let want = vec![1];
        let got = super::most_used(&input);
        assert!(equal_vector(&want, &got));
    }

    fn equal_vector(xs: &[u32], ys:&[u32]) -> bool {
        if xs.len() != ys.len(){
            return false;
        }
        for i in 0..xs.len(){
            if xs[i] != ys[i]{
                return false;
            }
        }
        return true;
    }
}