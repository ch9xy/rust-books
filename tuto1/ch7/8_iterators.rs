fn main () {
    let some_vec = vec![1,2,3,4,5,6,7];
    let mut a = some_vec.iter();

    println!("{:?}", a);

    let mut b = some_vec.iter().any(|&x| x > 4);
    println!("{}",b);

    let mut b = some_vec.iter().all(|&x| x > 4);
    println!("{}",b);

    let b = some_vec.iter().find(|&&x| x > 4);
    println!("{}",b.unwrap());

    let b = some_vec.iter().position(|&x| x > 4);
    println!("{}", b.unwrap());

    let b = some_vec.iter().rposition(|&x| x > 4);
    println!("{}", b.unwrap());

    let b = some_vec.iter().max();
    println!("the max value is: {}", b.unwrap());

    let b = some_vec.iter().min();
    println!("the min value is: {}", b.unwrap());

    let mut b = some_vec.iter().rev();
    println!("Even though it should print the reverse order it prints regular order. {:?}", b);
    println!("But it is processed as reverse. The first element is: {:?}", b.next());
}