use std::cell::RefCell;

fn main () {
    let a = RefCell::new(10);
    let b = a.borrow();
    let c = a.borrow();
    drop(b);
    drop(c);
    let d = a.borrow_mut();
    drop(d);
    println!("{:?}", a);


    let aa = RefCell::new(10);
    let mut bb = aa.borrow_mut();
    *bb = 15;
    drop(bb);
    println!("{:?}", aa);

}