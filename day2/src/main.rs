use std::env;
use std::fs;
use std::io::Read;


fn main() {
    println!("Hello, world!");
    let info = fs::read_to_string("src/strat.txt").expect("error cant read file");

    let pre_vec = info.split_terminator("\n").collect::<Vec<&str>>();

    println!("new pre sorted vec = {:?}", pre_vec);

    for el in pre_vec {
        let my_c: Vec<_> = el.chars().collect();
        println!(" first char = {} ", my_c[0]);
        println!(" sec char = {} ", my_c[2]);
    }
}
