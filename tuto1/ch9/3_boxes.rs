#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>), // by using "Option" we do not occupy heap spaces.
    Nil,
}

use List::{Cons, Nil};

fn main () {

    let list = Cons(1, Some(Box::new(Cons(2, Some(Box::new(Cons(3, None)))))));
    println!("{:?}", list);
}