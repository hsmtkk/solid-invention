enum LeftRight {
    Left,
    Right,
}

struct Sock {
    typ: String,
    leftright: LeftRight,
}

struct LeftRightCount {
    left: u32,
    right: u32,
}

fn count_pair(socks:&[Sock]) -> u32 {

}

#[cfg(test)]
mod tests {
    use crate::socks_pair::LeftRight::Left;
    use super::{LeftRight, Sock};

    #[test]
    fn test0(){
        let socks = vec![
            Sock::new("A", LeftRight::Right),
            Sock::new("A", LeftRight::Left),
            Sock::new("B", LeftRight::Right),
            Sock::new("A", LeftRight::Left),
            Sock::new("A", LeftRight::Right),
            Sock::new("B", LeftRight::Left),
            Sock::new("A", LeftRight::Left),
            Sock::new("C", LeftRight::Left),
        ];
        let want = 3;
        let got = super::count_pair(socks);
        assert_eq!(want, got);
    }
}