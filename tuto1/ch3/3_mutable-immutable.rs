let mut heap_num: Vec<i32> = vec![4,5,6];
let ref1: &Vec<i32> = &heap_num;
let ref2: &Vec<i32> = &heap_num;
println!("ref1: {:?}, ref2: {:?}",ref1, ref2);
let ref3: &mut Vec<i32> = &mut heap_num;

let mut heap_num: Vec<i32> = vec![4,5,6];
heap_num.push(68);

let ref1: &Vec<i32> = &heap_num;
let ref2: &Vec<i32> = &heap_num;

heap_num.push(68);