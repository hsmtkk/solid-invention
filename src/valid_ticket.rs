use regex::Regex;

struct Validator {
    original: String,
}

impl Validator {
    fn new(original: &str) -> Validator {
        Validator {
            original: original.to_string(),
        }
    }

    fn get_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        for i in 1..self.original.len() {
            let first = &self.original[..i];
            let second = &self.original[i..];
            let mut pattern = String::new() + first;
            pattern += ".";
            pattern += second;
            patterns.push(pattern);
        }
        patterns
    }

    fn validate(&self, word: &str) -> bool {
        if word.contains(&self.original){
            return true;
        }
        for pattern in self.get_patterns(){
            let re = Regex::new(&pattern).expect("new regex");
            if let Some(_) = re.find(word) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Validator;
    use maplit::hashmap;

    #[test]
    fn test_get_patterns() {
        let validator = Validator::new("paiza");
        let want = vec!["p.aiza", "pa.iza", "pai.za", "paiz.a"];
        let got = validator.get_patterns();
        assert_eq!(want, got);
    }

    #[test]
    fn test0() {
        let validator = Validator::new("paiza");
        let test_cases = hashmap! {
            "sdfpaizaoiu" => true,
        "sdfpaizoiu" => false,
        "paxiza" => true,
        "paxizya" => false,
        "ghepaizmakbn" => true,
        "paizzza" => false,
        "abcpadizzzaopq" => false,
        };
        for (word, want) in test_cases {
            let got = validator.validate(word);
            assert_eq!(want, got);
        }
    }
}
