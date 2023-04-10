println!("Enter a number");

let mut some_num = String::new();

std::io::stdin() Stdin
.read_line(&mut some_num) Result<usize, Error>
.expect("failed to read input");

let some_num = some_num.trim().parse().expect(msg:"invalid input");

if some_num != 0 {
    if some_num %2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }
} else {
    println!("The number is 0");
}