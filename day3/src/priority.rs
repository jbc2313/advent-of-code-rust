use std::collections::HashMap;
//use std::any::type_name;

/*
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
*/
pub fn letter_priority() -> (HashMap<char, i32>, HashMap<char, i32>) {
    let lower = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut lowercase: HashMap<char, i32> = HashMap::new();
    for idx in 1..=26 {
        //println!("idx = {}", idx);  
        lowercase.insert(lower[idx-1], idx.try_into().unwrap());
    }


    // not in order
    //println!("{:?}", lowercase);
    // not in order here either  hashmap retur in an arbitrary order...

    for (key, value) in &lowercase {
        println!("{key}: {value}");
    }

  
    let mut uppercase: HashMap<char, i32> = HashMap::new();
    

    for (key, value) in &lowercase{
        uppercase.insert(key.to_ascii_uppercase(),value + 26);
    };

    println!("{:?}", uppercase); 

       // for el in upper {
    //     //println!("{:?}", print_type_of(&el));
    //     //let el = &el.to_uppercase();
    //     println!("{:?}", el);
    //
    // }
    
    return (lowercase, uppercase);


}
