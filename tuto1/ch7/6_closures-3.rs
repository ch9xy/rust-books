

fn division<F: Fn(f32) -> bool>(x: f32, y:f32, f: F) {
    if f(y) == true {
        println!("The division result is: {}", x/y);
    } else {
        println!("The division is not possible");
    }
}


fn main () {

    let division_status = |y: f32| {if y != 0.0 {true} else {false} };

    division(5.0, 10.0, division_status);
    division(5.0, 0.0,division_status);

}