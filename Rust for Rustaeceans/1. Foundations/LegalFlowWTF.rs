fn main () {
    let mut x = Box::new(42);
    let r = &x;     // 'a

    if 0.6 /* rand() is pseudo */ > 0.5 {   
        *x = 84;
    } else {
        println!("{}", r);    // 'a
    }
    println!("{}", r);
}


// This is a code just to understand rust's lifetimes.