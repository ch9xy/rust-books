
use::ops::Deref;

impl Deref for MySmartPointer {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.value
    }
}

fn main () {
    let a = 50;
    let b = Box::new(a);
    println!("{}", 50 == a);
    println!("{}", 50 == *b);
    println!("{}", a == b); // WON't WORK   

    let sptr1 = MySmartPointer::new(a);
    let sptr2 = MySmartPointer::new(*b);
    println!("{}", a == *sptr1);
}