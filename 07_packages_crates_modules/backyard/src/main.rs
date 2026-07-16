// Create shortcut to Asparagus type. Then just need to write "Asparagus"
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};

    println!("I'm growing {plant:?}!");
}

// TODO: left off here: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#grouping-related-code-in-modules
