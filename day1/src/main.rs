use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::Read;


// fn read_file() -> std::io::Result<Vec<u8>> { 
//     let mut file = fs::File::open("input.txt")
//         .expect("file not found!");
//     let mut data = Vec::new();
//     file.read_to_end(&mut data);
//     return Ok(data);
// }


fn main() {
    println!("Hello, world!");
    let info = fs::read_to_string("src/input.txt").expect("error cant read file");
    //println!("{}", info.len());
    println!("{}", info);

    // this colects all the numbers with no spaces
    // i need a space betwwen the groups
    // let vec1 = info.split_whitespace().collect::<Vec<&str>>();
    // println!("vec 1 = {:?}", vec1);
    
    // this worked for what i was trying to do. collect all
    // the numbers in each line, seperated by the space between the groups of numbers
    

    let vec2 = info.split_terminator("\n").collect::<Vec<&str>>();
    //println!("vec 2 = {:?}", vec2);
    
    let mut vec3: Vec<i32> = vec![];
    let mut temp: i32;

    for val in vec2 {
        if val.len() > 2 {
            temp = val.parse().unwrap();
            vec3.push(temp);
        } else {
            vec3.push(0);
        }   
    };

   // println!("vec 3 = {:?}", vec3);
    println!("vec 3 length = {}", vec3.len());
    
    let mut temp2: i32 = 0;
    let mut added_vec: Vec<i32> = vec![];
    let end = vec3.len();
    let mut i = 0;

    for val in vec3 {
        i = i + 1;  
        if val == 0 
        {
          added_vec.push(temp2);
          temp2 = 0;
        } else {
           temp2 = val + temp2; 
        }
        if i == end {
            added_vec.push(temp2);
        }

    }
    //println!("Length of added-vec = {}", added_vec.len());
    //println!("added_vec = {:?}", added_vec);
    
   
    // SOLUTION FOR PART 1 
    // #######################
    let mut largest_num = 0;
    for val in &added_vec {
        if val > &largest_num {
            largest_num = *val;
        }
    
    };
    
    println!("Largest Num = {}", largest_num);
   
    added_vec.sort();
    added_vec.reverse();
    //println!("sorted list = {:?}", added_vec);
    
    let top3: Vec<i32> = vec![added_vec[0], added_vec[1], added_vec[2]];
    println!("TOP 3 = {:?}", top3);
    


    let mut largest_three: VecDeque<i32> = VecDeque::new();
    
    for num in added_vec {
        if Some(&num) > largest_three.back(){
            largest_three.push_back(num); 
        }else{
            largest_three.push_front(num);
        }
        if largest_three.len() > 3 {
            largest_three.pop_front();
        }
    }

    println!("Largest three numbers = {:?}", largest_three);
    println!("Added up = {}", largest_three[1] + largest_three[0] + largest_three[2]);

}
