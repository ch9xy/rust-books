fn main() {
    let x: f64 = 32.6;
    let y: f64 = x;

    println!("x: {}, y: {}", x, y);

    let s1: String = String::from("abc");
    let s2 = &String = &s1;
    println!("s1: {}, s2: {}", s1, s2);

    let vec_1: Vec<i32> = vec![5,6,7,8,9];
    let vec_2: Vec<i32> = vec_1.clone();
    println!("Vec 1: {:?}, Vec 2: {:?}",vec_1, vec_2);
}