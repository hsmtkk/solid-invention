#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Medal {
    gold:u32,
    silver:u32,
    bronze:u32,
}

impl Medal {
    fn new(gold:u32,silver:u32,bronze:u32) -> Medal {
        Medal{gold,silver,bronze}
    }
}

fn sort(medals:&[Medal]) -> Vec<Medal> {
    let mut copied = medals.to_vec();
    copied.sort_by(|a, b| b.cmp(&a));
    copied
}

#[cfg(test)]
mod tests {
    use super::Medal;
    #[test]
    fn test0(){
        let input = vec![
            Medal::new(3,5,9),
            Medal::new(15,20,35),
            Medal::new(30,45,72),
            Medal::new(15,20,31),
            Medal::new(27,33,59),
            Medal::new(27,35,77),
        ];
        let want = vec![
            Medal::new(30,45,72),
            Medal::new(27,35,77),
            Medal::new(27,33,59),
            Medal::new(15,20,35),
            Medal::new(15,20,31),
            Medal::new(3,5,9),
        ];
        let got = super::sort(&input);
        assert_eq!(want,got);
    }
}