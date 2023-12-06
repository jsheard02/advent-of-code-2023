use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::num::ParseIntError;
use std::println;
use std::collections::HashMap;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut total = 0;
        for line in lines {
            if let Ok(ip) = line {
               if let Ok(line_result) = part_two_solve(&ip) {
                   total += line_result;
               }
            }
        }
        println!("{}", total);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn part_one_solve(line: &str) -> Result<i32, ParseIntError> {
//    let colon = line.find(":");
//    match colon {
//        Some(value) => if game_is_possible(&line[value + 1..]) { line[5..value].parse::<i32>() } else { Ok(0) },
//        None => Ok(0)
//    }
// }

// fn game_is_possible(games: &str) -> bool {
 //    let parts = games.split(';');
 //    let mut result = true;
 //    
 //    'outer: for game in parts {
 //        for dice_output in game.split(',') {
 //            println!("{}", dice_output);
 //            if dice_output.ends_with("blue") && dice_output[1..dice_output.len() - 5].parse::<i32>().unwrap() > 14 {
 //                result = false;
 //                break 'outer;
 //            }
 //            else if dice_output.ends_with("green") && dice_output[1..dice_output.len() - 6].parse::<i32>().unwrap() > 13 {
 //                result = false;
 //                break 'outer;
 //            }
 //            else if dice_output.ends_with("red") && dice_output[1..dice_output.len() - 4].parse::<i32>().unwrap() > 12 {
 //                result = false;
 //                break 'outer;
 //            }
 //        }
 //    }
 //    return result;
 // }

fn part_two_solve(line: &str) -> Result<i32, ParseIntError> {
    let colon = line.find(":");
    match colon {
        Some(value) => calculate_game_power(&line[value + 1..]),
        None => Ok(0)
    }
}

fn calculate_game_power(games: &str) -> Result<i32, ParseIntError> {
    let parts = games.split(';');
    let mut max_cube_values = HashMap::from([
        ("blue", 1),
        ("green", 1),
        ("red", 1),
    ]);

    for game in parts {
        for dice_output in game.split(',') {
            if dice_output.ends_with("blue") { 
                let number_of_dice = dice_output[1..dice_output.len() - 5].parse::<i32>().unwrap();
                if number_of_dice > max_cube_values["blue"]
                {
                    max_cube_values.entry("blue").and_modify(|value| *value = number_of_dice);
                }
            }
            else if dice_output.ends_with("green") { 
                let number_of_dice = dice_output[1..dice_output.len() - 6].parse::<i32>().unwrap();
                if number_of_dice > max_cube_values["green"]
                {
                    max_cube_values.entry("green").and_modify(|value| *value = number_of_dice);
                }
            }
            else if dice_output.ends_with("red") { 
                let number_of_dice = dice_output[1..dice_output.len() - 4].parse::<i32>().unwrap();
                if number_of_dice > max_cube_values["red"]
                {
                    max_cube_values.entry("red").and_modify(|value| *value = number_of_dice);
                }
            }
        }
    }
    
    return Ok(max_cube_values["red"] * max_cube_values["blue"] * max_cube_values["green"])
}
