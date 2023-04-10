fn main () {

    let my_number = 5;
    let mut guess = false;


    println!("Guess my number which is between 1 and 20");

    while guess != true {
        let mut number = String::new();
        std::io::stdin()
        .read_line(&mut number)
        .expect("failed to read input");

        let number: u8 = number.trim().parse().expect("invalid input");

        if my_number == number {
            guess = true;
            println!("You guessed the number correctly");
        } else {
            println!("try again");
        }
    }


}