use std::collections::HashMap;

#[derive(Debug)]
struct BuyEvent {
    name: String,
    price: u32,
}

impl BuyEvent {
    fn new(name: &str, price: u32) -> BuyEvent {
        BuyEvent {
            name: name.to_string(),
            price,
        }
    }
}

fn sum_events(events: Vec<BuyEvent>) -> Vec<String> {
    let mut m: HashMap<String, u32> = HashMap::new();
    for e in events {
        match m.get(&e.name) {
            Some(price) => {
                let p = price + e.price;
                m.insert(e.name, p);
            }
            None => {
                m.insert(e.name, e.price);
            }
        }
    }
    let mut sorted = Vec::new();
    for (n, p) in m {
        sorted.push(BuyEvent::new(&n, p));
    }
    sorted.sort_by_key(|a| a.price);
    sorted.reverse();
    println!("{:?}", sorted);

    let mut results = Vec::new();
    for e in sorted {
        results.push(e.name);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::BuyEvent;

    #[test]
    fn test() {
        let events = vec![
            BuyEvent::new("A", 1000),
            BuyEvent::new("B", 1000),
            BuyEvent::new("B", 2000),
            BuyEvent::new("C", 2000),
        ];
        let want = vec!["B", "C", "A"];
        let got = super::sum_events(events);
        assert_eq!(want, got);
    }
}
