use std::collections::HashMap;

fn main() {
    // All keys need to have same type & all values need to be of same type for a given HashMap.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get a value from the HashMap
    let blue_score = scores
        .get(&String::from("Blue"))
        .copied() // to get the Option<i32> instead of Option<&i32>
        .unwrap_or(0); // handle the option (the "Some" value or 0)

    // Get a value from the HashMap with error handling
    let yellow_score = scores
        .get(&String::from("Yellow"))
        .ok_or_else(|| {
            // could return an error here instead of printing
            println!("Yellow score not found");
        });

    // Iterating over a HashMap's values ---------------------------------------------------------
    // For loop takes ownership - Need to use ref (&scores) so we can still use it afterwards
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership ---------------------------------------------------------------------------------
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // map owns field_name and field_value now, can't use them
    // anymore
    // println!("{field_name}"); can't do this - value borrowed after move error

    // Updating a HashMap ------------------------------------------------------------------------
    // 1. Overwriting a value:
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrites the previous value

    // print the value for "Blue"
    println!("{scores:?}");

    // 2. Adding a record only if doesn't yet exist
    // use "Entry" enum
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(60);

    // 3. Updating a value based on an old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); 
        *count += 1; // update it in the map
    }
    // or_insert gives mutable ref to current count (so if update it, updates it in the map).

    println!("{map:?}");
}
