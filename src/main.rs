mod puzzles;

use crate::puzzles::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            println!("No arguments provided");
        }
        _ => {
            let arg = args[1].as_str();
            let data = std::fs::read_to_string(format!("data/{}.txt", arg));
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
                    "day4" => {
                        day4::solve(data);
                    }
                    "day5" => {
                        day5::solve(data);
                    }
                    "day6" => {
                        day6::solve(data);
                    }
                    "day7" => {
                        day7::solve(data);
                    }
                    "day8" => {
                        day8::solve(data);
                    }
                    "day9" => {
                        day9::solve(data);
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
