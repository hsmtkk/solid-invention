struct TableGenerator {}

impl TableGenerator {
    fn new(header:Vec<&str>, contents:Vec<Vec<&str>>)-> TableGenerator {
        unimplemented!()
    }

    fn generate_table(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::TableGenerator;

    #[test]
    fn test0(){
        let want = read_file("test/table_generator/test0.txt");
        let header = vec!["id", "name"];
        let mut contents = Vec::new();
        contents.push(vec!["1", "ito"]);
        contents.push(vec!["2", "sakakibara"]);
        contents.push(vec!["3", "takahashi"]);
        let tg = TableGenerator::new(header, contents);
        let got = tg.generate_table();
        assert_eq!(want,got);
    }

    fn read_file(path:&str) -> String {
        let mut f = std::fs::File::open(path).expect("open file");
        let mut s = String::new();
        f.read_to_string(&mut s).expect("read file");
        s
    }
}