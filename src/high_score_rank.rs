struct Problem {
    parameter_nums: u32,
    user_nums: u32,
    top: u32,
    coefficients: Vec<f32>,
    items: Vec<Vec<u32>>,
}

fn read_problem() -> Problem {
    let mut f = std::fs::File::open("test/high_score_rank.txt").expect("open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("read to string");
    let lines = s.split("\n").collect();

    let elems = lines[0].split(" ").collect();

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::Problem;

    #[test]
    fn test_read_problem(){
        let got = super::read_problem();
        assert_eq!(4, got.parameter_nums);
        assert_eq!(10, got.user_nums);
        assert_eq!(3, got.top);
        assert_eq!(4, got.coefficients.len());
        assert_eq!(10, got.items.len());
        let want_items = vec![255, 150, 50, 65472];
        for i in 0..4 {
            assert_eq!(got[1][i], want_items[i]);
        }
    }
}