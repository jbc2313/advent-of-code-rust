use std::env;
use std::fs;
use std::io::Read;


fn main() {
    println!("Hello, world!");
    let info = fs::read_to_string("src/strat.txt").expect("error cant read file");

    let pre_vec = info.split_terminator("\n").collect::<Vec<&str>>();

    println!("new pre sorted vec = {:?}", pre_vec);

    let game_points = 0;

    for el in pre_vec {
        let my_c: Vec<_> = el.chars().collect();
        let opp_choice = my_c[0];
        let my_choice = my_c[2];
        //println!(" first char = {} ", opp_choice);
        //println!(" sec char = {} ", my_choice);

        if opp_choice == 'A' {
            //println!("A WAS SELECTED");
            if my_choice == 'X' {
                println!(" They CHOSE A, I CHOSE X");
            } else if my_choice == 'Y' {
               println!(" They CHOSE A, I CHOSE Y"); 
            } else if my_choice == 'Z' {
                println!("THEY CHOSE A, I chose z");
            }

        }

        if opp_choice == 'C' {
            match my_choice {
                'X' => {
                    println!("X followed B");
                    println!("You picked Rock");
                },
                'Y' => {
                    println!("Y followed B")
                },
                'Z' => {
                    println!("Z followed B")
                },
                _ => {
                    println!("You entered something wrong")
                },
           
            }
        };

        if opp_choice == 'C' { 
            match my_choice {
                'X' => println!("X followed C"),
                'Y' => println!("Y followed C"),
                'Z' => println!("Z followed C"),
                _ => println!("You entered something wrong"),
            }
        }
        
    }
}
