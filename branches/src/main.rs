fn main() {
    let x = 3;
    if x < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false")
    };

    println!("Hello, world!");

    let condition = true;
    let number = if !condition { 5 } else { 6 };
    println!("{number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("counting up");
        loop {
            if count == 5 {
                println!("We have reached the destination!!");
                break 'counting_up;
            }
            count += 1;
        }
    }
    println!("End count = {}", count);

    let a = [1, 2, 3, 5, 6];
    for element in a {
        println!("{element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFT OFF");

    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }
}
