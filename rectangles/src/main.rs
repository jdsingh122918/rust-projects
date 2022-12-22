#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rectangle1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("The area of rectangle is {}", rectangle1.area());
    println!("Rectangle: {:#?}", rectangle1);
    dbg!(&rectangle1);
}
