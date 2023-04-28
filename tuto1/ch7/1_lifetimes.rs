fn main () {
/
    let some_int = 10;

    let additional_int = some_fn(&some_int);
    println!("{}", additional_int);
    println!("{}", additional_int);
    println!("{}", some_int);
    println!("{}", some_int);



}
// Rust can not understand what the return reference should point to.
// function's return type contains a borrowed value, but there is no value for it to be borrowed from.
/* 
fn greater(i: &i32, j: &i32) -> &i32 {
    if i > j {
        i
    } else{
        j
    }
}
   */ 

fn some_fn(i: &i32) -> &i32 {
    &i
}