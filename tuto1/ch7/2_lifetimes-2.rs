fn main () {
    let s_1 = "Hello"; // type &str
    let v;
    {
        let s_2 = String::from("World"); // type String
        v = some_fn(s_1, s_2.as_str());
    }
    println!("{}", v);
}

fn some_fn<'a,'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
    first_str
}