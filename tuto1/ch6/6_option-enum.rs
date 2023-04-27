/* 

fn main () {
    let mut disease: Option<String> = None;
    disease = Some(String::from(:"Diabetes"));

    match disease {
        Some(disease_name) => println!("You have the disease of {}", disease_name),
        None => prnitln!("You do not have any disease"),
    }
}
*/

// ------


/* 

struct Person {
    name: String,
    age: i32,
}

fn main () {

    let s1 = Some("Some String");
    println!("The value of s1 is {:?} and the value of s1 itself after unwrapping is {:?}",s1, s1.unwrap());

    let f1 = Some(10.54);
    let mut f2 = 16.5;
    f2 = f2 + f1.unwrap();
    println!("The value of the sum is {}", f2);

    let v1 = Some(vec![0,1,2,3]);

    let p1 = Person {
        name: String::from("John"),
        age: 32,
    };

    let someone = Some(p1);

}

*/

fn square(num: Option<i32>) -> Option<i32> {
    match num {
        Some(number) => Some(number * number),
        None => None,
    }
}

fn main () {


    let number = Some(6);

    if square(number) != None {
        println!("The result of the square is {:?}", square(number).unwrap());
    } else {
        println!("We do not have any value");
    }
        square(None);

}