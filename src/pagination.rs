struct Problem {
    total_items: u32,
    item_per_page: u32,
    page_index: u32,
}

impl Problem {
    fn solve(&self) -> Vec<u32> {
        let mut results = Vec::new();
        let begin = (self.page_index-1)*self.item_per_page + 1;
        let end = self.page_index * self.item_per_page + 1;
        for i in begin..end {
            results.push(i);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;
    #[test]
    fn test0(){
        let p = Problem{total_items:34, item_per_page:10, page_index:3};
        let want = vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30];
        let got = p.solve();
        assert_eq!(want, got);
    }
}