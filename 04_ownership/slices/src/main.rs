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


    string_literals_as_slices();
    other_slices();
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

// Was: "fn first_word_slices(s: &String) -> &str {", more flexible if it's a &str instead of &String
// s: String -> coerces into &str
// can pass in string literals
// can pass any string slice
fn first_word_slices(s: &str) -> &str { // use a string slice instead (&str)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_literals_as_slices() {
    // https://doc.rust-lang.org/book/ch04-03-slices.html#string-literals-as-slices
    // string literals are slices
    let my_str = "howdy"; // my_str is a &str
    // &str is a slice (ref to portion or all of string data)


    let my_string = String::from("hello world");
    // first_word_slices works on slices of strings, whether partial or whole
    let word = first_word_slices(&my_string[0..6]);
    let word = first_word_slices(&my_string[..]);

    // first_word_slices also works references to `String`s, which are equivalent to whole slices
    // of `String`s
    let word = first_word_slices(&my_string);


    let my_string_literal = "hello world";
    // first_word_slices works on slices of string literals, partial and whole
    let word = first_word_slices(&my_string_literal[0..6]);
    let word = first_word_slices(&my_string_literal[..]);

    // because string literals are string slices, this works too
    let word = first_word_slices(my_string_literal);


    // String is like a notebook page you own — you can write on it,
    // erase it, make it longer.
    // &str is like someone handing you a sticky note with words already written on it 
    // — you can read them, but you can't change or add to them.
}

fn other_slices() {
    // slices work on things other than strings as well
    // this array:
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
