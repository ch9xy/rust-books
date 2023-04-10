
fn new_stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let popped_val = stack.pop();
    println!("The popped value is {:?}", popped_val);
    popped_val
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Can not add more.")
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("failed to read input");

    let n: u32 = n.trim().parse().expect("invalid input");
    n
}


fn main () {


    let input_string = String::from("Welcome to rust");
    let size_stack = input_string.len();
    let mut stack = new_stack(size_stack);

    let mut rev_string = String::new();
    for _i in input_string.chars(){
        push(&mut stack, _i, size_stack);
    }

        for _i in 0..size(&stack) {
            rev_string.push(pop(&mut stack).unwrap());
        }

        println!("The input string is {:?}", input_string);
        println!("The output string is {:?}", rev_string);
}
