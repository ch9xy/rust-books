fn main () {



    let mut var = 100;
    loop {
        var = var - 1;
        if var % 13 == 0 {
            break;
        }
    }
    println!("the highest number lesser than the given number divisible by 13 is {}", var);
    
}