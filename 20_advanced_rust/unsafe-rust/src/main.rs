use std::slice;

fn main() {
    println!("Hello, world!");
    // Unsafe rust:
    // 1. Dereference a raw pointer.
    // 2. Call an unsafe function or method.
    // 3. Access or modify a mutable static variable.
    // 4. Implement an unsafe trait.
    // 5. Access fields of unions.

    dereferencing_raw_pointer();
    creating_abscraction_over_unsafe_code();
}

fn dereferencing_raw_pointer() {
    // *const T
    // *mut T

    // can have both immutable and mutable pointers or mult mutable pointers to same location
    // aren't guaranteed to point to valid memory
    // are allowed to be null
    // don't implement any automatic cleanup




    // Create a mutable and immutable raw pointer ----------------------------------------------
    let mut num = 5;

    let r1 = &raw const num; // use &raw keyword
    let r2 = &raw mut num;

    // Need an unsafe block to dereference a raw pointer:
    unsafe {
        println!("r1 is {}", *r1); // deref with *
        println!("r2 is {}", *r2);
    }
    // notice: have both a mutable and immutable reference to same location (num) ^^
    // ------------------------------------------------------------------------------------------




    // raw pointer who's validity we can't be so certain of:
    let address = 0x012345usize;
    let r = address as *const i32;
}

// Trying to implement something like split_at_mut with safe code but won't work:
// fn my_split_at_mut_wont_work(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//
//     assert!(mid <= len);
//
//     ( &mut values[..mid], &mut values[mid..] )
// }

// implement something like split_at_mut that will work (need unsafe code)
fn my_split_at_mut_fixed_with_unsafe_code(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr: *mut i32 = values.as_mut_ptr(); // get raw pointer to slice

    assert!(mid <= len); // ensure things are safe moving forward

    // from_raw_parts_mut takes in raw pointer and must trust that the pointer is valid - unsafe
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn creating_abscraction_over_unsafe_code() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];

    let (first_set, second_set) = my_split_at_mut_fixed_with_unsafe_code(&mut nums, 3);

    println!("first part: {first_set:?}");
    println!("second part: {second_set:?}");
}
