fn main () {

    let single_value = Box::new(0.625); // single_value stored on stack whereas the value 0.625 is stored on heap
    let x = 0.625;
    println!("Are the values equal ? {}", x == *single_value);


    let mut stack_var = 4;
    
    let heap_var = Box::new(stack_var);

    stack_var = 5;

    println!("The value of stack_var is: {} the value of heap_var is {}", stack_var, heap_var); // 5, 4

    let point = Box::new((100,125));

    println!("{} {}", point.0, point.1);

    let a = Box::new(150);
    println!("{}", *a);
    println!("{}", a);
    
}