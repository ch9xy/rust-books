
fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_val: &mut Vec<u32> = stack.pop();
    println!("The popped value is {:?}", popped_val);
    popped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Can not add more.")
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin
    .read_line(&mut n)
    .expect("failed to read input");

    let n: u32 = n.trim().parse().expect("invalid input");
    n
}


fn main () {


println!("Let us first create a stack for our use");
println!("Please enter the size of the stack:");
let size_stack = input();
let mut stack = new_stack(size_stack as usize);

loop{
println!("\n\n ****** Menu ****** \n");
println!("1. Push \n 2. Pop \n 3. Display \n 4. Size \n 5. Exit");
println!("\n Enter your choice: ");
let choice = input();

match choice {
    1 => {
        println!("Enter the value to be inserted: ");
        let item = input();
        push(stack, item, size_stack as usize);
    }

    2 => println!("The element popped is: {:?} ", pop(&mut stack)),
    3 => println!("The elements are: {:?} ", stack),
    4 => println!(size(&stack)),
    5 => break,
    _ => println!("Wrong selection try again"),

    
}
     println!("Do you want to continue ? 1 = Yes / 2 = No ");
     let status = input();
     if status == 1 {
        continue;
     } else {
        break;
     }


}


}
