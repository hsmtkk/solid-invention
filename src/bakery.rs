use anyhow::Result;
use std::collections::HashMap;

type Price = u32;

#[derive(PartialEq, Eq, Hash)]
struct Bread {
    price: u32,
}

impl Bread {
    fn new(price:u32) -> Bread {
        Bread{price}
    }
}

struct Bakery {
    bread_stock_map: HashMap<Bread, Price>,
}

impl Bakery {
    fn new(bread_stock_map:HashMap<Bread, Price>) -> Bakery {
        Bakery{bread_stock_map}
    }

    fn buy(&self, bread_count_map:HashMap<Bread, u32>) -> Result<Price> {
        unimplemented!()
    }

    fn bake(&self, bread_count_map:HashMap<Bread, u32>){
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use super::Bread;
    use super::Bakery;

    #[test]
    fn test0(){
        let bread0 = Bread::new(150);
        let bread1 = Bread::new(200);
        let bakery = Bakery::new(hashmap!{bread0=>2, bread1=>3});
        assert_eq!(550, bakery.buy(hashmap!{bread0=>1, bread1=>2}).unwrap());
        if let Ok(price) = bakery.buy(hashmap!{bread0=>1, bread1=>2}) {
            // should fail
            assert!(false);
        }
        bakery.bake(hashmap!{bread0=>0, bread1=>1});
        assert_eq!(400, bakery.buy(hashmap!{bread0=>0, bread1=>2}).unwrap());
    }
}