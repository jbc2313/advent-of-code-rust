use std::env;
use std::fs;
use std::io::Read;

fn main() {

    //pull in the txt file and parse data into a Vec 
    let info = fs::read_to_string("src/inputday3.txt").expect("error cant read file");
    let pre_vec = info.split_terminator("\n").collect::<Vec<&str>>();
    println!("new pre sorted vec = {:?}", pre_vec);
    
    //need to split each element in half, directly in the middle. the length of all is an even
    //number so they will split in two easily.
    for el in pre_vec {
        println!("Length of element = {:?}", el.len());
    }

}
