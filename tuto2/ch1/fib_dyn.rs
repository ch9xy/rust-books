use std::collections::HashMap;

fn fib_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let value = fib_dyn(n-1, map) + fib_dyn(n-2, map);
                map.insert(n, value);
                value
            }
        }
    }
}

fn main () {
    let mut a = HashMap::new();
    for i in 1..41 {
    println!("{}: {}", i, fib_dyn(i, &mut a));
}
}