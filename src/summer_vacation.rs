struct Span {
    begin: u32,
    end: u32,
}

impl Span {
    fn new(begin:u32, end:u32) -> Span {
        Span{begin,end}
    }

    fn include(&self, day:u32) -> bool {
        self.begin <= day && day <= self.end
    }
}

fn solve(spans: Vec<Span>) -> bool {
    for d in 1..31 {
        let mut found = true;
        for span in &spans {
            if !span.include(d){
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::Span;
    use super::solve;

    #[test]
    fn test0(){
        let spans = vec![
            Span::new(16, 19),
            Span::new(14, 17),
        ];
        assert!(solve(spans));
    }

    #[test]
    fn test1(){
        let spans = vec![
            Span::new(22, 23),
            Span::new(17, 20),
            Span::new(14,19),
        ];
        assert!(!solve(spans));
    }

    #[test]
    fn test2(){
        let spans = vec![
            Span::new(1, 10),
            Span::new(2, 9),
            Span::new(4, 7),
            Span::new(3, 13),
            Span::new(5, 6),
            Span::new(1, 6),
            Span::new(2, 8),
            Span::new(4, 5),
            Span::new(4, 7),
            Span::new(3, 15),
        ];
        assert!(solve(spans));
    }

}