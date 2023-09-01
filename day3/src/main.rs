use std::{fs};

mod priority;

fn main() {

    //pull in the txt file and parse data into a Vec 
    let info = fs::read_to_string("src/inputday3.txt").expect("error cant read file");
    let pre_vec = info.split_terminator("\n").collect::<Vec<&str>>();
    //println!("new pre sorted vec = {:?}", pre_vec);
    
    // pull in the scoring hashmaps
    let (lower, upper) = priority::letter_priority();

    //need to split each element in half, directly in the middle. the length of all is an even
    //number so they will split in two easily.
    let mut needle_vec: Vec<char> = Vec::new();
    for el in pre_vec {
        //println!("Length of element = {:?}", el.len());
        let el_length = el.len();
        let split_point = el_length / 2;
        let (front, back) = el.split_at(split_point);
    //    println!("front({:?}) = {:?}\nback({:?}) = {:?}\n", front.len(), front, back.len(), back);

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
    //                println!("NEEDLE MATCHES EL:{:?}", el);
                    matched = true;
                    break;
                }
            }
            if matched == true {
     //           println!("LOOP FINISHED NEEDLE FOUND");
                break;
            }
        }
        //println!("NEEDLE = {:?}", needle);
        needle_vec.push(needle);
        //println!("========================");
    }

    println!("LIST OF NEEDLES = {:?}", needle_vec);
    println!("READY to MOVE on?");
    println!("==========================");

    let mut point_counter: i32 = 0; 
    for el in needle_vec {
        let is_upper = el.is_ascii_uppercase();
        //println!("{:?} is upper", is_upper);
        if is_upper {
            for (key, val) in upper.clone().into_iter() {
                if el == key {
                    //println!("{:?} == {:?} increment counter", el, key);
                    point_counter += val; 
                };
            };
        }else{
            //Two ways to do the same thing
            for (key, val) in &lower {
                if el == *key {
                   // println!("{:?} == {:?} increment counter", el, key);
                    point_counter += val;
                };
                
            };
        };
    };

    println!("Point Counter = {:?}", point_counter);


    println!("========PART 2=================");
// part 2
    
    // pre_vec is the data set in a vector

    let pre_vec2 = info.split_terminator("\n").collect::<Vec<&str>>();
    //println!("{:?}", pre_vec2);
    let pre_vec2_length = pre_vec2.len();
   
// need to make a vec of vecs
    let length = pre_vec2.len();
    let num = pre_vec2.len() / 3;
    let mut count_to_three = 1;
    let mut first_group: Vec<char> = Vec::new();
    let mut second_group: Vec<char> = Vec::new();
    let mut third_group: Vec<char> = Vec::new();
    let mut matching_items: Vec<char> = Vec::new();
    for idx in 0..=pre_vec2_length-1 {
        if count_to_three == 1 {
            first_group = pre_vec2[idx].chars().collect::<Vec<char>>();
        };
        if count_to_three == 2 {
            second_group = pre_vec2[idx].chars().collect::<Vec<char>>();
        };
        if count_to_three == 3 {
            third_group = pre_vec2[idx].chars().collect::<Vec<char>>();
            //compare 3 groups here
            
            //println!("First group = {:?}", first_group);
            //println!("Sec group = {:?}", second_group);
            //println!("Third group = {:?}", third_group);
            let group1_size = first_group.len();
            let group2_size = second_group.len();
            let group3_size = third_group.len();
            
            for idx1 in 0..=group1_size-1 {
                let mut hit = false; 
     
                for idx2 in 0..=group2_size-1 {
                    if first_group[idx1] == second_group[idx2] {
                        for idx3 in 0..=group3_size-1 {
                            if third_group[idx3] == second_group[idx2] &&
                                third_group[idx3] == first_group[idx1] && second_group[idx2]==first_group[idx1] {
                        //        println!("hit = {:?}", third_group[idx3]);
                                matching_items.push(third_group[idx3]); 
                                hit = true;
                            
                            };

                            if hit == true {
                                break;
                            };
                        };

                        if hit == true {
                            break;
                        };
                    };
                };
                if hit == true {
                    break;
                };
            };
        };


        if count_to_three == 3 {
            count_to_three = 1;
            continue;
        };
        count_to_three += 1;
    }
    println!("number of bags = {:?}", length);
    println!("number of matches should be {:?}", num); 
    println!("actual number of matching = {:?}", matching_items.len());
    println!("matching items below");
    println!("{:?}", matching_items);

    let mut counter2 = 0;

    
    for el in matching_items {
        let is_upper = el.is_ascii_uppercase();
        //println!("{:?} is upper", is_upper);
        if is_upper {
            for (key, val) in upper.clone().into_iter() {
                if el == key {
                    //println!("{:?} == {:?} increment counter", el, key);
                    counter2 += val; 
                };
            };
        }else{
            //Two ways to do the same thing
            for (key, val) in &lower {
                if el == *key {
                   // println!("{:?} == {:?} increment counter", el, key);
                    counter2 += val;
                };
                
            };
        };
    };

    println!("Total points = {:?}", counter2);
}
