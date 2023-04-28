fn main () {
    let int1 = 5;
    let int2 = 10;
    let result = greater(&int1, &int2);
}

fn greater(i: &i32, j: &i32) -> i32 {
    if i > j {
        *i
    } else {
        *j
    }
}