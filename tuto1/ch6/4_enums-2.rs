enum Conveyance{
    Car(i32),
    Train(i32),
    Air(i32),
}

impl Conveyance {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            Conveyance::Car(miles) => *miles as f32 * 14.0,
            Conveyance::Train(miles) => *miles as f32 * 18.0,
            Conveyance::Air(miles) => *miles as f32 * 30.0,
        };
        allowance
    }
}

fn main() {
    let participant_1 = Conveyance::Car(25);
    let participant_2 = Conveyance::Train(50);
    let participant_3 = Conveyance::Air(100);
    println!("The allowance is: {}", participant_1.travel_allowance());
    println!("The allowance is: {}", participant_2.travel_allowance());
    println!("The allowance is: {}", participant_3.travel_allowance());

}
