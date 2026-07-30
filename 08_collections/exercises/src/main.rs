fn main() {
    let even_num_list = vec![10, 2, 12, 5, 1, 11]; // [1, 2, 5, 10, 11, 12]
    let odd_num_list = vec![3, 4, 1, 2, 5]; // []

    // find median
    let median_even = find_median(&even_num_list); // should be 2
    let median_odd = find_median(&odd_num_list); // should be 3

    println!("median_even: {}", median_even);
    println!("median_odd: {}", median_odd);

    // find mode
    // let mode = find_mode(&nums);
}

fn find_median(nums: &[i32]) -> i32 {
// fn find_median(nums: &mut Vec<i32>) -> i32 {
    let mut sorted = nums.to_vec();
    sorted.sort();

    let length = nums.len();
    let mid = length / 2;

    if length % 2 == 0 { // if is even
        (nums[mid - 1] + nums[mid]) / 2 // average the 2 middle values (returns)
    } else {
        nums[mid] // not even, just return the middle one in the list
    }
}

