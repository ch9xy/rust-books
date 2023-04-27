

fn division(divident: f64, divisor: f64) -> Result<f64, String> {
    
    
    /* 
    if divisor == 0.0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(divident / divisor) // We need to wrap the result
    }
    */
    
    match divisor {
        0.0 => Err(String::from("Error: division by zero")),
        _ => Ok(divident / divisor),
    }

}


fn main () {
    println!("{:?}", division(4.0,2.0));

    let some_vec = vec![5,3,2,5,1,2,5,6];
    
    let result1 = match some_vec.get(5) {
        Some(a) => Ok(a),
        None => Err("The value doesn't exist")
    };

    println!("The value of the result1 is: {:?}", result1);


}