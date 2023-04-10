struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32
}

impl Person {


    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 /3.) * 0.5
    }
}

fn main () {


    let person1 = Person{
        name: String::from("John"),
        citizenship: String::from("USA"),
        age: 40,
        gender: 'M',
        salary: 40_000,
    };

    println!("The taxes on Person {} is {}", person1.name, Person::compute_taxes(&person1));
//println!("The taxes on Person {} is {}", person1.name, person1.compute_taxes());

    let person2 = Person{
        name: String::from("Bob"),
        ..person1
};

    // Tuples

    struct Numbers(i32,i32);

    impl Numbers {
        fn greater(&self) -> i32 {
            if self.0 >= self.1 {self.0} else {self.1}
        }
        fn lesser(&self) -> i32 {
            if self.0 <= self.1 {self.0} else {self.1}
        }
    }

    let some_nums = Numbers(32,16);

    // No borrow problems occur. All fine.
    println!("The values of the two fields are: {}, {}", some_nums.0, some_nums.1);
    println!("The values of the two fields are: {}, {}", some_nums.0, some_nums.1);
   
    println!("The greater one from some_nums is : {}", some_nums.greater());
    println!("The lesser one from some_nums is : {}", some_nums.lesser());
}