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
        
        // ROCK
        if opp_choice == 'A' {
            if my_choice == 'Z' {
                println!("They CHOSE A, I CHOSE X");
                println!("2 Point for X(PAPER) + 6 points for WIN");
                my_game_points = my_game_points + 8;
            } else if my_choice == 'Y' {
               println!("They CHOSE A, we CHOSE Y");
               println!("1 points for Y(ROCK) + 3 points for DRAW");
               my_game_points = my_game_points + 4;
            } else if my_choice == 'X' {
                println!("--- THEY CHOSE A, I chose z ---");
                println!("3 points for Z(scissors) + 0 points for losing");
                my_game_points = my_game_points + 3;
            }

        }
        
        // scissors
        if opp_choice == 'C' {
            match my_choice {
                'Z' => {
                    println!("---X followed C----");
                    println!("1 point for X(rock) + 6 points for WIN");
                    my_game_points = my_game_points + 7;
                },
                'Y' => {
                    println!("--- Y followed C ----");
                    println!("3 points for Y(scissors) + 3 points for DRAW");
                    my_game_points = my_game_points + 6;
                },
                'X' => {
                    println!("Z followed C");
                    println!("2 points for Z(paper), 0 Point for LOOSE");
                    my_game_points = my_game_points + 2;
                },
                _ => {
                    println!("--- You entered something wrong ---")
                },
           
            }
        };
        
        // paper
        if opp_choice == 'B' { 
            match my_choice {
                'Z' => {
                    println!("X followed B");
                    println!("3 Point for X(scissor), 6 for WIN");
                    my_game_points = my_game_points + 9;
                },
                'Y' => {
                    println!("---Y followed B---");
                    println!("2 points for Y(paper)+ 3 point for DRAW");
                    my_game_points = my_game_points + 5;
                },

                'X' => {
                    println!("---Z followed B---");
                    println!("1 points for Z(rock) + 0 point for LOOSE");
                    my_game_points = my_game_points + 1;
                },
                _ => {println!("You entered something wrong")},
            }
        }
        
    };


    println!("TOTAL POINTS = {}", my_game_points);
    // 9374 IS WRONG TRY AGAIN..... :/
    // 15605 is WRONG
    // PART 1 ANSWER = 15337
    // PART 2 ANSWER = 
}
