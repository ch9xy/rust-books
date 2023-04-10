

fn main() {
let some_number = 100;

match some_number {
    1 => println!("The number is 1"),
    2 => println!("The number is 2"),
    4..=100 => println!("The number is between 4 and 100"),
    _ => println!("The number is greater than 100"),
}


}