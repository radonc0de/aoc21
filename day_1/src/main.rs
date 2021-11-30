use std::fs;
use std::env::args;

fn main() {
    
    //IMPORT PUZZLE INPUT
    let args: Vec<String> = args().collect();
    let file = &args[1];
    let input = fs::read_to_string(file)
        .expect("Failed to read file to String");
    let organized = input.lines().collect::<Vec<&str>>(); // split into &str of each line
    //let organized = input.split('/').collect::<Vec<&str>>(); // split by arg into &str
    //let organized = input.split_whitespace().collect::<Vec<&str>>(); // split by whitespace into &str
    println!("{:?}", organized);

}

