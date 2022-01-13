use std::net::IpAddr;

fn is_valid_ip_address(s: &str) -> bool {
    match s.parse::<IpAddr>(){
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    #[test]
    fn test0() {
        let patterns = hashmap! {
            "192.168.0.1" => true,
            "192.168.0.2" => true,
            "192.168.0.3" => true,
            "192.168.0.4" => true,
            "192.400.1.10.1000..." => false,
            "4.3.2.1" => true,
            "0..33.444..." => false,
            "1.2.3.4" => true,
        };
        for (s, want) in patterns {
            let got = super::is_valid_ip_address(s);
            assert_eq!(want, got);
        }
    }
}
