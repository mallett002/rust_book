use std::{collections::HashMap, ops::Index};

fn main() {
    // let even_num_list = vec![10, 2, 12, 5, 2, 1, 11];
    // let odd_num_list = vec![3, 5, 4, 1, 2, 5];
    //
    // // find median
    // let median_even = find_median(&even_num_list);
    // let median_odd = find_median(&odd_num_list);
    //
    // println!("median_even: {}", median_even);
    // println!("median_odd: {}", median_odd);
    //
    // // find mode
    // // let mode = find_mode(&nums);
    // let mode_even = find_mode(&even_num_list); // should be 2
    // let mode_odd = find_mode(&odd_num_list); // should be 3
    //
    // println!("mode_even: {}", mode_even);
    // println!("mode_odd: {}", mode_odd);

    // Convert strings to pig-latin
    println!("\n Convert strings to pig-latin");
    let my_str = String::from("This is going to be fun!");
    let my_str_in_pl = to_pig_latin(&my_str);
    println!("{my_str_in_pl}");
}

fn find_median(nums: &[i32]) -> i32 {
    // fn find_median(nums: &mut Vec<i32>) -> i32 {
    let mut sorted = nums.to_vec();
    sorted.sort();
    println!("sorted: {sorted:?}");

    let length = sorted.len();
    let mid = length / 2;

    if length % 2 == 0 {
        // if is even
        (sorted[mid - 1] + sorted[mid]) / 2 // average the 2 middle values (returns)
    } else {
        sorted[mid] // not even, just return the middle one in the list
    }
}

// Mode - The one that occurs the most
fn find_mode(nums: &[i32]) -> i32 {
    let mut count_map = HashMap::new();
    let mut highest_count: i32 = 0;
    let mut current_leader: i32 = 0;

    for &num in nums {
        let count = count_map.entry(num).or_insert(0); // put in map if no yet there
        *count += 1; // increment it's count

        if *count > highest_count {
            highest_count = *count;
            current_leader = num;
        }
    }

    current_leader
}

fn find_mode_with_option(nums: &[i32]) -> Option<i32> {
    let mut count_map = HashMap::new();

    // step 1: build the map
    for &num in nums {
        *count_map.entry(num).or_insert(0) += 1; // same as doing in 2 lines like find_mode does
    }

    // step 2: find the num that occurs most in the map
    count_map
        .into_iter() // loop over entries
        .max_by_key(|&(_, count)| count) // get the key that has the highest count
        .map(|(num, _)| num) // receives the pair from max_by_key and returns the key
}

fn to_pig_latin(input_str: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let length = input_str.chars().size_hint();

    let mut result = String::new();

    for (index, word) in input_str.to_string().split_whitespace().enumerate() {
        let trimmed = word.trim_end_matches(|c: char| c.is_ascii_punctuation());

        let end_punctuation = &word[trimmed.len()..];
        let suffix = if end_punctuation.is_empty() { " " } else { end_punctuation };

        let first_char: char = trimmed.chars().next().unwrap();
        let first_is_vowel: bool = vowels.contains(&first_char);

        let base_word = &trimmed[1..];

        let stem = match first_is_vowel {
            true => format!("{base_word}-hay"),
            false => format!("{base_word}-{first_char}ay")
        };

        result.push_str(&stem);
        result.push_str(suffix);
    }

    result
}

