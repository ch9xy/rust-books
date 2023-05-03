pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main () {
    let mut n = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };

    for i in n {
        println!("{}", i);
    }
}