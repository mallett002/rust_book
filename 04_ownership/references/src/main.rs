fn main() {
    references();
    try_modify_borrowed_reference(); // you can't, unless mutable
    only_one_mut_ref();
    ref_scope_duration();
    dangling_refs();
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
    let s = String::from("hello");
    change(&s);

    // but, can us mutable reference and modify value
    let mut s = String::from("hello");
    change_mutable_ref(&mut s);
}

// this won't work - can't modify a borrowed value
// similar to immutible vars
fn change(some_string: &String) {
    some_string.push_str(", world");
}

fn change_mutable_ref(some_string: &mut String) {
    some_string.push_str(", world");
}

fn only_one_mut_ref() {
    let mut s = String::from("hello");

    // If have a mut ref, can't create other references until the mut ref is out of scope
    let r1 = &mut s;
    let r2 = &mut s; // can't create another

    println!("{r1}, {r2}");




    // But can use curly brackets to create new scope
    let mut s1 = String::from("howdy");
    
    {
        let r1 = &mut s1;
    } // r1 goes out of scope here, so can make new ref no with problems

    let r2 = &mut s1;




    // can't combine mutable & immutable references of same value (same address)
    let mut my_str = String::from("hello");

    let ref1 = &my_str; // no problem
    let ref2 = &my_str; // no problem
    let ref3 = &mut my_str; // BIG PROBLEM

    println!("{ref1}, {ref2}, {ref3}");
}

fn ref_scope_duration() {
    // ref in scope from when ref created until last usage
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1}, {r2}"); 
    // r1 & r2 used, out of scope now

    let r3 = &mut s; // no problem - can create mutable reference now
    println!("{r3}");
}

fn dangling_refs() {
   let reference_to_nothing = dangle(); 
}

// tries to return a reference to a string
// by the time the caller uses this reference, what it's referencing gets dropped
fn dangle() -> &String {
    let s = String::from("hello"); 

    &s
} // s out of scope and is dropped


