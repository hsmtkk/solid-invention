use itertools::Itertools;

struct Authenticator {
    secret: String,
}

impl Authenticator {
    fn new(secret:&str) -> Authenticator{
        Authenticator{secret:secret.to_string()}
    }

    fn authenticate(&self, word:&str) -> bool {
        if self.secret == word {
            return false;
        }
        for chs in self.secret.chars().into_iter().permutations(3) {
            let s: String = chs.iter().collect();
            if s == word {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Authenticator;
    #[test]
    fn test(){
        let auth = Authenticator::new("abc");
        assert!(auth.authenticate("bac"));
        assert!(!auth.authenticate("abc"));
        assert!(!auth.authenticate("xy"));
    }
}