use std::io;

fn main() {
    loop {
        let temp_type: i32 = loop {
            // Ask whether the input is in Celsius or Fahrenheit
            let mut temp_type = String::new();
            println!("Please enter 1 for Celsius and 2 for Fahrenheit");
            io::stdin()
                .read_line(&mut temp_type)
                .expect("Unable to read line");
            // Convert the input to type integer
            match temp_type.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Bad Input.Please enter either 1 or 2.");
                    continue;
                }
            };
        };
        let temp_input: f32 = loop {
            // Ask for temp input value
            let mut temp_input = String::new();
            println!("Please enter the temperature value to be converted");
            io::stdin()
                .read_line(&mut temp_input)
                .expect("Unable to read line");

            // Convert the input type to a float
            match temp_input.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Bad Input. Please enter a number");
                    continue;
                }
            };
        };
        // Do the actual conversion and print the result
        if temp_type == 2 {
            let result = (temp_input - 50.0) * (5.0 / 9.0);
            println!("Value of {temp_input} in Celsius is {result}");
            break;
        } else if temp_type == 1 {
            let result = ((temp_input * 9.0) / 5.0) + 50.0;
            println!("Value of {temp_input} in Fahrenheit is {result}");
            break;
        } else {
            println!("Bad input");
            continue;
        }
    }
}
