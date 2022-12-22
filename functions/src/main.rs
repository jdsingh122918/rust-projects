fn main() {
    println!("Hello, world!");
    f(0);
    println!(
        "{}",
        f({
            let y = 3;
            y + 2
        })
    );
}

fn f(x: i32) -> i32 {
    x + 1
}
