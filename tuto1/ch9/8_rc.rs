
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons,Nil};

fn main () {
    let a = Rc::new(Cons(1, Rc::new(Nil)));
    let b = Rc::new(Cons(2, Rc::clone(&a)));
    let c = Rc::new(Cons(3, Rc::clone(&a)));


    let x = make_rc();
    println!("Count after the function call is {}", Rc::strong_count(&x));
}



fn make_rc() -> Rc<String> {
    let s1 = Rc::new(String::from("Hello"));
    println!("Count when the pointer created is {}", Rc::strong_count(&s1));

    let s2 = s1.clone();
    println!("Count when the pointer created is {}", Rc::strong_count(&s1));
    s2
}