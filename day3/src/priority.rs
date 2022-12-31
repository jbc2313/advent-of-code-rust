use std::collections::HashMap;
use std::any::type_name;

pub fn letter_priority() {
    let lower = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let mut lowercase: HashMap<String, i32> = HashMap::new();
    for idx in 1..=26 {
        //println!("idx = {}", idx);  
        lowercase.insert(String::from(lower[idx-1]), idx.try_into().unwrap());
    }

    // not in order
    //println!("{:?}", lowercase);
    // not in order here either  hashmap retur in an arbitrary order...

    for (key, value) in &lowercase {
        println!("{key}: {value}");
    }
    
    // need to change lowercase letters to uppercase.
    // then add them to a hashmap like the above.
    
    let mut upper = &lower;

    // for el in upper {
    //     println!("{:?}", print_type_of(el));
    //
    // }
    
    let mut uppercase: HashMap<String, i32> = HashMap::new();



}
