use std::ops::Deref;

struct MySmartPointer<T: std::fmt::Debug>{value: T}

impl<T: std::fmt::Debug> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer{value: x}
    }
}

impl<T: std::fmt::Debug> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: std::fmt::Debug> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer object from memory {:?}", self.value);
    }
}

fn my_fn(str: &str) {
    println!("The string received from the main is {}", str);
}

fn main () {

    let sptr_1 = MySmartPointer::new("John");
    my_fn(&sptr_1); // &MySmartPointer -> &String -> &str
    

    let some_vec = MySmartPointer::new(vec![1,2,3]);
    
    for z in &*some_vec {
        println!("The value is {}", z);
    }

    

}