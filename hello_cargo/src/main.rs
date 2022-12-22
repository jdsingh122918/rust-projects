use crate::point::Point;

pub mod point {
    #[derive(Debug)]
    pub struct Point(pub i32, i32);

    impl Point {
        pub fn origin() -> Self  {
            Point(0, 0)
        }
    }
}

fn main() {
    let mut p = Point::origin();
    p.0 += 1;
    println!("{:?}", p);

    let mut v = vec![100, 200, 300];
    for n_ref in &mut v {
        *n_ref += 100;
    }
    println!("{:?}", v);

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10;
        println!("{i}")
    }
}
