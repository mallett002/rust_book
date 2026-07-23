mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// TODO: left off: https://doc.rust-lang.org/book/ch08-00-common-collections.html
