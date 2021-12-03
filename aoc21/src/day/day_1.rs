pub fn main(input: String) -> (i32, i32){
    let organized = input.lines().collect::<Vec<&str>>(); // split into &str of each line
    
    //PARSE TO INTEGER
    let mut nums = vec![];
    for i in organized {
        nums.push(i.parse::<i32>().unwrap());
    }

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
   return (0, sum);
}
