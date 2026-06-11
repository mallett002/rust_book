fn main() {
    // Demonstrating "move"
    let s1 = String::from("hello");
    let s2 = s1; // move
    // println!("{s1}"); // Error can't do this (this has "moved")
    println!("{s2}");

    // vars no longer needed are dropped
    let mut s = String::from("hello"); // allocation moved (dropped) once reassigned below
    println!("{s} world");
    s = String::from("ahoy!"); // causes above "s" to be dropped
    println!("{s} world");




    // Ownership with functions ------------------------
    let s = String::from("howdy"); // s comes into scope
    takes_ownership(s); // s moves into func takes_ownership, s no longer valid here
    // println!("can still use {s}"); // would cause error

    let x = 5;
    makes_copy(x);

    // x is an i32 which implements Copy. Ownership doesn't move into makes_copy. Can still use:
    println!("can still use {x}");




    // Return values and scope --------------------------
    let s1 = gives_ownership(); // s1 becomes the owner

    let s2 = String::from("hello"); // s2 comes into scope

    // s2 moved into takes_and_gives_back
    let s3 = takes_and_gives_back(s2); // s3 comes into scope 



    // Return multiple values, using a tuple
    let my_str = String::from("quack");
    let (my_str_again, len) = calc_length(my_str);

    println!("The length of {my_str_again} is {len}.");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("takes_ownership: {some_string}"); // the owner, can use some_string
} // some_string goes out of scope, drop is called, memory freed

fn makes_copy(some_int: i32) { // some_int comes into scope
    println!("makes_copy: {some_int}"); // the owner, can use some_int
} // stored on stack, nothing special happens


// Will move its return value into the func that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
   a_string // returned and moved into function that called this takes_and_gives_back
}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
