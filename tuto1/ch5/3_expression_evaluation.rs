// TO BE CONTINUED

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


fn individual_symbols(input_expr: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();

    let input_chars: Vec<char> = input_expr.chars().collect();
    let mut temp: Vec<char> = Vec::new();

    for i in input_chars {
        if i != '+' && i != '-' && i != '*' && i != '/' && i != '^' && i != '(' && i != ')' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                tokenized_input.push(i.to_string());
            } else {
            tokenized_input.push(temp.into_iter().collect());
            tokenized_input.push(i.to_string());
            temp = vec![];
        }
    } 
}
println!("{:?}", tokenized_input);
tokenized_input
}

fn main () {

    let input_expr = String::from("(33+45/3*(2+9)-50)");
    println!("The original expression is: {:?}", input_expr);
    individual_symbols(input_expr);



}
