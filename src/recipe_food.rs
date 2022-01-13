use std::collections::HashMap;

type MaterialCount = HashMap<String, u32>;

fn cook(recipe:&MaterialCount, my_materials:&MaterialCount) -> u32 {
    let mut max_count = 0;
    loop {
        if enough_materials(recipe, my_materials, max_count) {
            max_count += 1;
        } else {
            break;
        }
    }
    max_count - 1
}

fn enough_materials(recipe:&MaterialCount, my_materials:&MaterialCount, count:u32) -> bool {
    for (material, number) in recipe {
        let required = *number * count;
        match my_materials.get(material){
            Some(having) => {
                if required > *having {
                    println!("{} {} {}", material, required, having);
                    return false;
                }
            },
            None => {
                return false;
            },
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    #[test]
    fn test0(){
        let recipe: super::MaterialCount = hashmap!{
            "supaisu".to_string() => 5,
            "imo".to_string() => 2,
            "niku".to_string() => 2,
            "mizu".to_string() => 3,
        };
        let my_materials: super::MaterialCount = hashmap!{
            "mizu".to_string() => 7,
            "imo".to_string() => 4,
            "ninjin".to_string() => 10,
            "unagi".to_string() => 6,
            "supaisu".to_string() => 20,
            "niku".to_string() => 5,
        };
        let want = 2;
        let got = super::cook(&recipe, &my_materials);
        assert_eq!(want, got);
    }
}