use std::collections::HashMap;

pub fn letter_priority() {
    let lower = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let mut lowercase: HashMap<String, i32> = HashMap::new();
    for idx in 1..=26 {
        //println!("idx = {}", idx);  
        lowercase.insert(String::from(lower[idx-1]), idx.try_into().unwrap());
    }

    println!("{:?}", lowercase);



}
