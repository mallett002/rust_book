fn main() {
    // slice is sequence of elements in collection
    // slice is type of reference, so does not have ownership

    // demonstrate problem they solve:
    let mut my_str = String::from("hello world");

    let word_index = first_word(&my_str);
    my_str.clear(); // string is now ""
    // problem:  word_index & my_str get out of sync


    // enter string slices:
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}, {world}");


    // more slices

    // these are equal:
    let slice = &s[0..2];
    let slice = &s[..2]; // if at start, can just drop the 0


    // these are equal also
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..]; // if at end, can drop ending index

    // can drop both values to get whole string
    let slice = &s[0..len];
    let slice = &s[..];

    // first_word using slices
    let word = first_word_slices(&s);

    // mut and immut refs can't exist at same time:
    // s.clear(); // needs mutable reference
    println!("the first word is: {word}"); // immutable reference

    // left off: https://doc.rust-lang.org/book/ch04-03-slices.html#string-literals-as-slices
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
