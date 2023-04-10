#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]



fn stack_function(mut stack_num:i32){
    stack_num = 56;
    println!("Var: {}", stack_num);
}

fn heap_function(var: &Vec<i32>) {
    //var.push(50);
    println!("Var: {:?}", var);
}

fn main () {

    let stack_num: i32 = 32;
    let mut heap_vec: Vec<i32> = vec![4,5,6];

    stack_function(stack_num);
    println!("The value inside the main of stack_num: {}", stack_num);

    heap_function(&mut heap_vec);
    println!("The value inside the main of heap_vec: {:?}", heap_vec);
}