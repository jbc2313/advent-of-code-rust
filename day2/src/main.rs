use std::env;
use std::fs;
use std::io::Read;


fn main() {
    println!("Hello, world!");
    let info = fs::read_to_string("src/strat.txt").expect("error cant read file");

    let pre_vec = info.split_terminator("\n").collect::<Vec<&str>>();

    println!("new pre sorted vec = {:?}", pre_vec);

    let mut my_game_points = 0;

    for el in pre_vec {
        let my_c: Vec<_> = el.chars().collect();
        let opp_choice = my_c[0];
        let my_choice = my_c[2];
        //println!(" first char = {} ", opp_choice);
        //println!(" sec char = {} ", my_choice);

        // lines with --- are none srategy choices

        if opp_choice == 'A' {
            if my_choice == 'X' {
                println!("--- They CHOSE A, I CHOSE X ---");
            } else if my_choice == 'Y' {
               println!("They CHOSE A, we CHOSE Y");
               println!("2 points for Y(paper) + 6 points for win");
               my_game_points = my_game_points + 8;
            } else if my_choice == 'Z' {
                println!("--- THEY CHOSE A, I chose z ---");
            }

        }

        if opp_choice == 'C' {
            match my_choice {
                'X' => {
                    println!("---X followed C----");
                    println!("---You picked Rock---");
                },
                'Y' => {
                    println!("--- Y followed C ----")
                },
                'Z' => {
                    println!("Z followed C");
                    println!("3 points for Z(scissors), 3 Point for DRAW");
                    my_game_points = my_game_points + 6;
                },
                _ => {
                    println!("--- You entered something wrong ---")
                },
           
            }
        };

        if opp_choice == 'B' { 
            match my_choice {
                'X' => {
                    println!("X followed B");
                    println!("1 Point for X(Rock), 0 for losing");
                    my_game_points = my_game_points + 1;
                },
                'Y' => {println!("---Y followed B---")},
                'Z' => {println!("---Z followed B---")},
                _ => {println!("You entered something wrong")},
            }
        }
        
    };


    println!("TOTAL POINTS = {}", my_game_points);
    // 9374 IS WRONG TRY AGAIN..... :/
}
