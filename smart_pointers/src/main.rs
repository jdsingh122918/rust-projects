#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let test_list = Cons(1, Box::new(Nil));
    let b = &test_list;
    println!("b = {:?}", *b);
}