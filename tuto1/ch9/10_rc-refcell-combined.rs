use std::cell::RefCell;
use std::rc::Rc;

fn main () {
    let a = Rc::new(RefCell::new(String::from("Java")));
    let b = Rc::clone(&a);
    *b.borrow_mut() = String::from("c++");
    println!("{:?}", b);
}