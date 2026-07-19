// Create shortcut to Asparagus type. Then just need to write "Asparagus"
use crate::garden::vegetables::Asparagus;

// declare garden module (found in src/garden.rs)
pub mod garden;

fn main() {
    let plant = Asparagus {};

    println!("I'm growing {plant:?}!");
}

