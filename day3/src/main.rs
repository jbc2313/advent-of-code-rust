use std::env;
use std::fs;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    let info = fs::read_to_string("src/inputday3.txt").expect("error cant read file");

    let pre_vec = info.split_terminator("\n").collect::<Vec<&str>>();

    println!("new pre sorted vec = {:?}", pre_vec);
}
