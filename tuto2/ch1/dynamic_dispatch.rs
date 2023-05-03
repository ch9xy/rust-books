use std::fmt::Display;

fn show_all(v: Vec<&dyn Display>) {
    for item in v {
        println!("{}", item);
    }
}

fn main () {
    let v = vec![&12 as &Display, &"Hi!" as &Display];

    show_all(v);
}