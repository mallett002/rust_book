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

    references();
    try_modify_borrowed_reference();
    only_one_mut_ref();
    ref_scope_duration();

    // left off: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references
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

fn references() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // pass in reference

    println!("Length of {s1} is {len}");

    // s -> (ptr s1) -> val -> (heap val)
}

// takes in reference
// reference is address to value owned by other var
fn calculate_length(s: &String) -> usize { // s reference to a string (borrowing)
    s.len()
} // s out of scope
// doesn't own it, so s is not dropped

fn try_modify_borrowed_reference() {
    // let s = String::from("hello");
    // change(&s);

    let mut s = String::from("hello");
    change_mutable_ref(&mut s);
}

// this won't work - can't modify a borrowed value
// similar to immutible vars
fn change(some_string: &String) {
    // some_string.push_str(", world");
}

fn change_mutable_ref(some_string: &mut String) {
    some_string.push_str(", world");
}

fn only_one_mut_ref() {
    let mut s = String::from("hello");

    // If have a mut ref, can't create other references until the mut ref is used
    let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1}, {r2}");


    // But can use curly brackets to create new scope
    let mut s1 = String::from("howdy");
    
    {
        let r1 = &mut s1;
    } // r1 goes out of scope here, so can make new ref no with problems

    let r2 = &mut s1;



    // can't combine mutable & immutable refs
    let mut my_str = String::from("hello");

    let ref1 = &my_str; // no problem
    let ref2 = &my_str; // no problem
    // let ref3 = &mut my_str; // BIG PROBLEM

    println!("{ref1}, {ref2}");
}

fn ref_scope_duration() {
    // ref in scope from when ref created until last usage
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1}, {r2}"); // used, out of scope now

    let r3 = &mut s; // no problem
    println!("{r3}");
}
