use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway_1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway_1,
    );

    let user_pref2 = None;
    let giveaway_2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway_2,
    );

    more_closures();
}

fn more_closures() {
    inferring_types();
    ownership();

    // TODO: left off https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures
}

fn inferring_types() {
    // inferring types
    let add_one = |x: u32| x + 1;

    // no types
    let example_closure = |x| x;

    // gets type String from here:
    let s = example_closure(String::from("hello"));

    // cannot use an int now:
    // let n = example_closure(5); // already got the String type on it
}

fn ownership() {
    /* Closure can capture values in 3 ways (same 3 ways functions take params):
    - Borrow immutably
    - Borrow mutably
    - Taking ownership*/

    // Closure decides which option to take!
    immutable_ref();
    mutable_ref();
    move_ownership();
}

// only immutable borrow needed here, so that's what compiler does:
fn immutable_ref() {
    // Only borrows immutably:
    let list = vec![1, 2, 3];
    println!("Before defining closure {list:?}");

    let only_borrows = || println!("From closure {list:?}");

    println!("Before calling closure {list:?}");
    only_borrows();
    println!("After calling closure {list:?}");
}

// Since we're mutating the list, a mutable reference is needed here
fn mutable_ref() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure {list:?}");

    let mut borrows_mutably = || list.push(4); // captures mutable ref to list here

    // can't use list here - borrowed as mut ref
    // no other borrows allowed when there's a mutable borrow
    // println!("Before calling closure {list:?}");
    borrows_mutably();
    println!("After calling closure {list:?}");
}

// a move of ownership is needed here to ensure list is valid whole lifetime of new thread
fn move_ownership() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure {list:?}");

    // "move" keyword: force the closure to take ownership of list
    thread::spawn(move || println!("From closure {list:?}"))
        .join()
        .unwrap();

    // can't use list now - moved into spawn's closure
    // println!("After calling closure {list:?}");
}
