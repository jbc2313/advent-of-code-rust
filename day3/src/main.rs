use std::env;
use std::fs;
use std::io::Read;

mod priority;

fn main() {

    //pull in the txt file and parse data into a Vec 
    let info = fs::read_to_string("src/inputday3.txt").expect("error cant read file");
    let pre_vec = info.split_terminator("\n").collect::<Vec<&str>>();
    //println!("new pre sorted vec = {:?}", pre_vec);
    
    // pull in the scoring hashmaps
    let (upper, lower) = priority::letter_priority();

    //need to split each element in half, directly in the middle. the length of all is an even
    //number so they will split in two easily.
    let mut needle_vec: Vec<char> = Vec::new();
    for el in pre_vec {
        //println!("Length of element = {:?}", el.len());
        let el_length = el.len();
        let split_point = el_length / 2;
        let (front, back) = el.split_at(split_point);
        println!("front({:?}) = {:?}\nback({:?}) = {:?}\n", front.len(), front, back.len(), back);

        // need to find the one letter front and back both have, and get the score of that letter.

        //let el_list [_;0] = [];
        let front_el_vec = front.chars().collect::<Vec<char>>();
        let back_el_vec = back.chars().collect::<Vec<char>>();
        let mut needle: char = '?'; 
        let mut matched: bool = false;
        for char in &front_el_vec {

            needle = *char;
            for el in &back_el_vec {
                if needle == *el {
                    println!("NEEDLE MATCHES EL:{:?}", el);
                    matched = true;
                    break;
                }
            }
            if matched == true {
                println!("LOOP FINISHED NEEDLE FOUND");
                break;
            }
        }
        println!("NEEDLE = {:?}", needle);
        needle_vec.push(needle);
        println!("========================");
    }

    println!("LIST OF NEEDLES = {:?}", needle_vec);





}
