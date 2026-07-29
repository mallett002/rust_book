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

    // Iterating over a HashMap's values
    // For loop takes ownership - Need to use ref (&scores) so we can still use it afterwards
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
