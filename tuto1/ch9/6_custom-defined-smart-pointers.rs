use std::ops::Deref;

struct MySmartPointer{value: i32}

impl MySmartPointer {
    fn new(x: i32) -> MySmartPointer {
        MySmartPointer{value: x}
    }
}

impl Deref for MySmartPointer {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.value
    }
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer object from memory {:?}", self.value);
    }
}

fn main () {


    let a = 50;
    let b = Box::new(a);

    
    let sptr5 = MySmartPointer::new(a-22);
    let sptr1 = MySmartPointer::new(a);
    let sptr2 = MySmartPointer::new(*b);
    println!("{}", a == *sptr1);

    let sptr3 = MySmartPointer::new(a+3);
    let sptr4 = MySmartPointer::new(a+5);
    drop(sptr5);
    
    

}