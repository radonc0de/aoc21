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
    
    //PARSE TO INTEGER
    let mut nums = vec![];
    for i in organized {
        nums.push(i.parse::<i32>().unwrap());
    }
   println!("{:?}", nums);

   let mut a = 0;
   let mut b = 0;
   let mut sum = 0;

   let index = 3;
   for i in index..nums.len() {
        a = nums[i-1] + nums[i-2] + nums[i-3];
        b = nums[i] + nums[i-1] + nums[i-2];
        if b > a {
            sum += 1;
        }
   }

   println!("{}", sum);



}

