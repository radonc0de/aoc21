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
    let organized = input.split_whitespace().collect::<Vec<&str>>(); // split by whitespace into &str
    println!("{:?}", organized);
    
    //PARSE TO INTEGER
    /*
    let mut nums = vec![];
    for i in organized {
        nums.push(i.parse::<i32>().unwrap());
    }
    */
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let mut next = -1;
    for i in organized {
        match i {
            "forward" => next = 0,
            "up" => next = 1,
            "down" => next = 2,
            _       => {
                if next == 0 {
                    let temp = i.parse::<i32>().unwrap();
                    x += temp;
                    y += temp*aim;
                }else if next == 1{
                    aim -= i.parse::<i32>().unwrap();
                }else if next == 2{
                    aim += i.parse::<i32>().unwrap();
                }
            }

        }
    }
    println!("{}", x*y);


}
