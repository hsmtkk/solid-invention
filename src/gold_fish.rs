struct Problem {
    pois: u32,
    poi_power: u32,
    fish_weights: Vec<u32>,
}

impl Problem {
    fn new(pois:u32, poi_power:u32, fish_weights:&[u32]) -> Problem {
        Problem{pois, poi_power, fish_weights:fish_weights.to_vec()}
    }

    fn solve(&self) -> u32 {
        let mut fishes = 0;
        let mut pois = self.pois;
        let mut poi_power = self.poi_power;
        let mut fish_index = 0;
        loop {
            let w = self.fish_weights[fish_index];
            if poi_power > w {
                poi_power -= w;
                fishes += 1;
                fish_index += 1;
            } else {
                poi_power = self.poi_power;
                pois -= 1;
                if pois <= 0 {
                    break;
                }
            }
        }
        fishes
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;
    #[test]
    fn test0(){
        let p = Problem::new(2, 10, &vec![5,5,3,1,4]);
        let want = 4;
        let got = p.solve();
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let p = Problem::new(2, 4, &vec![4,3,5,2,2]);
        let want = 0;
        let got = p.solve();
        assert_eq!(want, got);
    }

}