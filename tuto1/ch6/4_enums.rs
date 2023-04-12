enum Conveyance{
    Car,
    Train,
    Air,
}

fn main() {
    let participant_1 = Conveyance::Car; // output is 0
    let participant_1 = Conveyance::Train; // 1
    let participant_1 = Conveyance::Air; // 2
    println!("The value of the option is {}", participant_1 as i32);
}