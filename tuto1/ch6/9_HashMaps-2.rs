use std::{collections::HashMap, hash::Hash};


fn main () {

    let mut likes:HashMap<&str, &str> = HashMap::new();


//likes.insert("John", "Apple");
//likes.insert("John", "Banana");
//println!("The fruit which is being liked is: {:?}", likes);
// OVERRIDES

likes.entry("John").or_insert("Apple");
likes.entry("John").or_insert("Banana");
println!("The fruit which is being liked is {:?}", likes);
// DOES NOT OVERRIDE

let some_vec = vec![5,5,8,1,0,5,5,5,5];
let mut freq_vec:HashMap<i32, u32> = HashMap::new();

for i in &some_vec {
    let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
    *freq += 1;
}

println!("{:?}", freq_vec);

}