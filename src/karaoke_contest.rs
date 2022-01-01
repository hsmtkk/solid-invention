type Song = Vec<u32>;

struct Problem {
    theme: Song,
    tried_songs: Vec<Song>,
}

impl Problem {
    fn new(theme: Song, tried_songs: Vec<Song>) -> Problem {
        Problem { theme, tried_songs }
    }

    fn solve(&self) -> u32 {
        let mut max_score = 0;
        for tried_song in &self.tried_songs {
            let score = self.calculate_score(tried_song);
            if score > max_score {
                max_score = score
            }
        }
        max_score
    }

    fn calculate_score(&self, tried_song: &Song) -> u32 {
        let mut score = 100;
        for i in 0..self.theme.len() {
            let want = self.theme[i];
            let tried = tried_song[i];
            let diff = abs(want, tried);
            if diff <= 5 {
                score -= 0;
            } else if diff <= 10 {
                score -= 1;
            } else if diff <= 20 {
                score -= 2;
            } else if diff <= 30 {
                score -= 3;
            } else {
                score -= 5;
            }
        }
        score
    }
}

fn abs(a: u32, b: u32) -> u32 {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;
    use super::Song;

    #[test]
    fn test0() {
        let want = 97;
        let theme: Song = vec![400, 410, 420];
        let tried_songs: Vec<Song> = vec![vec![400, 400, 400], vec![300, 300, 300]];
        let problem = Problem::new(theme, tried_songs);
        let got = problem.solve();
        assert_eq!(want, got);
    }
}
