struct MutStr<'a, 'b> {
    s: &'a mut &'b str
}

fn main () {
    let mut s = "hello";
    *MutStr { s: &mut s}.s = "world";

    println!("{}", s);
}