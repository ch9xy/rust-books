fn main () {
    let mut some_vec = vec![45,30,85,90,41,39];

    //for i in some_vec.iter() {    THEY ARE SAME
    for i in &some_vec{
        println!("{}", i);
    }

    println!("{:?}",some_vec);
}