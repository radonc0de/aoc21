use std::fs;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let day = &args[1];
    let path = format!("./input/{}", args[1]);
    let input = fs::read_to_string(path)
        .expect("Failed to read file to String");

    let parts = {
        match day.as_str() {
            "1" => aoc_21::day::day_1::main(input),
            "2" => aoc_21::day::day_2::main(input),
          //"3" => aoc_21::day::day_3::main(input),
          //"4" => aoc_21::day::day_4::main(input),
            _   => panic!("Unknown input"),
        }
    };
    println!("Day {} Part 1: {:?}", day, parts.0);
    println!("Day {} Part 2: {:?}", day, parts.1);
    
}
