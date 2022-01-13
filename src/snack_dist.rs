struct Problem {
    snacks: u32,
    num_containers: Vec<u32>,
}

impl Problem {
    fn new(snacks: u32, num_containers: &[u32]) -> Problem {
        Problem {
            snacks,
            num_containers: num_containers.to_vec(),
        }
    }

    fn solve(&self) -> u32 {
        let mut answers: Vec<Answer> = Vec::new();
        for num_container in &self.num_containers {
            let d = self.snacks / num_container;
            let m = self.snacks % num_container;
            let ans = Answer::new(self.snacks, *num_container, d, m);
            answers.push(ans);
        }
        answers.sort_by(compare_answer);
        answers[0].num_container
    }
}

struct Answer {
    snacks: u32,
    num_container: u32,
    division: u32,
    modulo: u32,
}

fn compare_answer(a: &Answer, b: &Answer) -> std::cmp::Ordering {
    if a.modulo < b.modulo {
        std::cmp::Ordering::Less
    } else if a.modulo > b.modulo {
        std::cmp::Ordering::Greater
    } else {
        if a.num_container > b.num_container {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl Answer {
    fn new(snacks: u32, num_container: u32, division: u32, modulo: u32) -> Answer {
        Answer {
            snacks,
            num_container,
            division,
            modulo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;

    #[test]
    fn test0() {
        let p = Problem::new(7, &vec![2, 3, 4]);
        let want = 3;
        let got = p.solve();
        assert_eq!(want, got);
    }

    #[test]
    fn test1() {
        let p = Problem::new(15, &vec![12, 13, 7, 5, 8]);
        let want = 5;
        let got = p.solve();
        assert_eq!(want, got);
    }
}
