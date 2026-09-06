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
    fn_once_example();
    fn_mut_example();
}

fn more_closures() {
    inferring_types();
    ownership();
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

    let mut borrows_mutably = || list.push(4); // captures mutable ref on "list" here

    // can't use list here - borrowed as mut ref
    // no other borrows allowed when there's a mutable borrow
    // println!("Before calling closure {list:?}");
    borrows_mutably();
    println!("After calling closure {list:?}");
}

// a move of ownership is needed here to ensure list is valid whole lifetime of new thread
fn move_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure {list:?}");

    // "move" keyword: force the closure to take ownership of list
    thread::spawn(move || println!("From closure {list:?}"))
        .join()
        .unwrap();

    // can't use list now - moved into spawn's closure
    // println!("After calling closure {list:?}");
}

/* Closure traits:
 - FnOnce - Moves captured values out of its body - can only be called once (like .unwrap_or_else on Option)
 - FnMut - Can be called multiple times and may mutate the value that is captured in closure
 - Fn - called more than once & don't mutate captured values (implements FnMut - any Fn is also
 Fn mut)

 Fn ⊂ FnMut ⊂ FnOnce: ("⊂" means "is subset of") (ex: cat ⊂ feline ⊂ mammal)
    - FnOnce  accepts: FnOnce, FnMut, Fn
    - FnMut   accepts: FnMut, Fn
    - Fn      accepts: Fn

Broadest (most general)
│
▼
FnOnce    ← any closure, but can only be called ONCE
│
FnMut     ← can be called MULTIPLE times, may mutate captured state
│
Fn        ← can be called MULTIPLE times, NO mutation
│
▲
Narrowest (most restrictive)
*/

// 1. Ex for FnOnce (how unwrap_or_else is defined on Option):
enum MyOption<T> {
    None,
    Some(T),
}

// how unwrap_or_else is defined on Option:
impl<T> MyOption<T> {
    pub fn unwrap_or_else_definition<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            MyOption::Some(x) => x,
            MyOption::None => f(),
        }
    }
}

fn fn_once_example() {
    // Ex 1. See above snippet for how unwrap_or_else is defined (implements FnOnce)

    // Ex 2. FnMut closure attempting to do FnOnce things:
    let mut list = [
        Rectangle {
            width: 5,
            height: 2,
        },
        Rectangle {
            width: 10,
            height: 29,
        },
        Rectangle {
            width: 7,
            height: 3,
        },
    ];

    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("closure called");

    // Issue here: sort_by_key uses FnMut but requires FnOnce for this to work
    list.sort_by_key(|r| {
        // sort_by_key implements FnMut, but FnOnce is needed (need to be able to call mult times)
        // 2nd iteration, "value" won't be there bc it was moved
        // sort_operations.push(value); // captures value, sends ownership to the sort_operations vec
        r.width 
    });

    println!("{list:#?}");

    // Ex 3. Fix Ex 2:
    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1; // captures mut ref to counter; can be called more than once
        r.width 
    });

    println!("{list:#?}, sorted in {num_sort_operations} operations");
}

// 2. Ex. for FnMut with .sort_by_key
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn fn_mut_example() {
    let mut list = [
        Rectangle {
            width: 5,
            height: 2,
        },
        Rectangle {
            width: 10,
            height: 29,
        },
        Rectangle {
            width: 7,
            height: 3,
        },
    ];

    // Uses FnMut closure bc called on list mult times (once for each item in list)
    list.sort_by_key(|r| r.width);

    // pretty print it with the :#?
    println!("{list:#?}");
}
