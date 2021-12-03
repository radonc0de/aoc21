pub fn main(input: String) -> (i32, i32){
    //PARt 2 SOLUTION
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let mut dir = 0; // 0=FORWARD, 1=UP, 2=DOWN 
    
    for i in input.split_whitespace() {
        match i {
            "forward" => dir = 0,
            "up" => dir = 1,
            "down" => dir = 2,
            _       => {
                match dir {
                    0 => {
                        let temp = i.parse::<i32>().unwrap();
                        x += temp;
                        y += temp*aim;
                    },
                    1 => aim -= i.parse::<i32>().unwrap(),
                    2 => aim += i.parse::<i32>().unwrap(),
                    _ => {}
                }
            }
        }
    }
    return (0, x*y);
}
