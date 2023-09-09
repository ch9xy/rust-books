fn main () {
    let mut x = Box::new(42);
    let r = &x;     // 'a

    if rand() /* rand() is pseudo. Put 0.6 or 0.4 to test. */ > 0.5 {   
        *x = 84;
    } else {
        println!("{}", r);    // 'a
    }
    // println!("{}", r); Activate this line and code will not compile.
}


// This is a code just to understand rust's lifetimes.
