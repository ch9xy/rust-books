fn main () {
    let mut some_vec = vec![45,30,85,90,41,39];


    // iter() or iter_mut() takes the reference.
    for i in some_vec.iter_mut() {
        *i += 5;
        println!("{}", i);
    }

    println!("{:?}",some_vec);
}