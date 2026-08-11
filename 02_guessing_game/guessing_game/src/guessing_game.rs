pub struct Guess {
    value: i32,
}

// Guess type for ensuring value between 1 and 100
// And ensuring that Guess.value is private - can only get created through the Guess::new fn
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
