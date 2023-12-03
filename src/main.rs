use std::fs;

mod puzzles;

pub use crate::puzzles::day0;
pub use crate::puzzles::day1;
pub use crate::puzzles::day2;
pub use crate::puzzles::day3;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            println!("No arguments provided");
        }
        _ => {
            let arg = args[1].as_str();
            let data = fs::read_to_string(format!("data/{}.txt", arg));
            match data {
                Ok(data) => match arg {
                    "day0" => {
                        day0::solve(data);
                    }
                    "day1" => {
                        day1::solve(data);
                    }
                    "day2" => {
                        day2::solve(data);
                    }
                    "day3" => {
                        day3::solve(data);
                    }
                    _ => {
                        println!("Invalid argument: {}", args[1]);
                    }
                },
                Err(error) => {
                    println!("Error reading data: {}", error);
                }
            }
        }
    }
}
