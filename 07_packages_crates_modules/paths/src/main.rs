use std::collections::HashMap; // bring the whole hashmap into scope (idiomatic for structs/enums

// use std::fmt;
// use std::io;
// use std::io::Result as IoResult; // use the "as" keyword to rename
// use std::cmp::Ordering;
// import std crates together:
use std::{
    cmp::Ordering,
    fmt,
    io::{self, Result as IoResult}, // std::io & std::io::Result
};

fn main() {
    let mut map = HashMap::new();

    map.insert(1, 2);
}

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

fn function3() -> IoResult<()> {
    // --snip--
    Ok(())
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// re-exporting front_of_house::hosting as public for consumers of this package to use
pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
