use std::io;

fn main() {
    let input: i32 = loop {
        // Get a valid input
        println!("Please enter the number you want to calculate the fibonacci number of");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");
        match input.trim().parse() {
            Ok(num) => {
                if num < 0 {
                    println!("Bad Input. Number should be greater than zero");
                    continue;
                } else {
                    break num;
                }
            }
            Err(_) => {
                println!("Bad Input. Please try again");
                continue;
            }
        }
    };
    println!("Result = {}", calculate_fibonacci(input));
}

fn calculate_fibonacci(input: i32) -> i32 {
    match input {
        0 => 0,
        1 | 2 => 1,
        number => calculate_fibonacci(number - 1) + calculate_fibonacci(number - 2),
    }
}
