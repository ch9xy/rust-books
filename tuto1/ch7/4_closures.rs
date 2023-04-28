fn main () {

    let x = 5;
    let square = |num: i32| println!("The square of the value is {}", num*num);
    square(x);
    let square = |num: i32| println!("The cube of the value is {}", num*num*num); // OVERRIDES THE OTHER CLOSURE
    square(x);

    let y = 15;
    square(y);

}