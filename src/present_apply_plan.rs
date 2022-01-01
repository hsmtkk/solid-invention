#[derive(Debug)]
struct Problem {
    n: u32,
    x: u32,
    y: u32,
}

impl Problem {
    fn new(n: u32, x: u32, y: u32) -> Problem {
        Problem { n, x, y }
    }
}

#[derive(Debug, PartialEq)]
enum Present {
    A,
    B,
    AB,
    N,
}

fn solve(p: Problem) -> Vec<Present> {
    let mut result = Vec::new();
    for i in 1..p.n+1 {
        let a = i % p.x == 0;
        let b = i % p.y == 0;
        if a && b {
            result.push(Present::AB);
        } else if a {
            result.push(Present::A);
        } else if b {
            result.push(Present::B);
        } else {
            result.push(Present::N);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::Present;
    use super::Problem;
    #[test]
    fn test0() {
        let p = Problem::new(5, 2, 4);
        let want = vec![Present::N, Present::A, Present::N, Present::AB, Present::N];
        let got = super::solve(p);
        assert_eq!(&want, &got);
    }

    #[test]
    fn test1() {
        let p = Problem::new(6, 3, 2);
        let want = vec![
            Present::N,
            Present::B,
            Present::A,
            Present::B,
            Present::N,
            Present::AB,
        ];
        let got = super::solve(p);
        assert_eq!(&want, &got);
    }
}
