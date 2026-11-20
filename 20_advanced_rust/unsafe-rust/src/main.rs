use std::thread;
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
    calling_external_code();
    mutable_static_variables();
    unsafe_traits();
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

// Implement something like split_at_mut that will work (need unsafe code).
// The function itself isn't unsafe and can be called from regular rust code (abstraction around unsafe rust).
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

fn misuse_of_unsafe_code() {
    let address = 0x01234usize; // just get a randon memory address
    let ptr = address as *mut i32; // get mutable pointer

    let values: &[i32] = unsafe {
        slice::from_raw_parts_mut(ptr, 1000)
    };
}

fn creating_abscraction_over_unsafe_code() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];

    let (first_set, second_set) = my_split_at_mut_fixed_with_unsafe_code(&mut nums, 3);

    println!("first part: {first_set:?}");
    println!("second part: {second_set:?}");
}


// "C" here means use the C application binary interface (ABI)
// some functions in ABIs are safe, can mark them with the safe keyword - don't need unsafe blocks to use
// https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#using-extern-functions-to-call-external-code
unsafe extern "C" { 
    fn abs(input: i32) -> i32;
}

fn calling_external_code() {
    // Allow rust to interact with code written in another language
    // FFI (Foreign Function Interface)
    // More info on this: https://doc.rust-lang.org/reference/items/external-blocks.html#abi

    unsafe {
        println!("absolute value of -3 according to c: {}", abs(-3));
    }
}

#[unsafe(no_mangle)] // don't let rust change the func name when compiled
pub extern "C" fn call_from_c() {
    println!("Just called a rust function from C!");
}

// static = global in rust
static SCREAMING_SNAKE_CASE: &str = "Hello, World!";

static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread is unsafe and is undefined behavior.
/// You *must* call it from a single thread at a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// #[allow(static_mut_refs)]
fn mutable_static_variables() {
    // mutable static variables are unsafe in rust
    unsafe {
        // SAFETY: This is only called from a single thread - main.
        add_to_count(3);
        println!("Counter: {}", *(&raw const COUNTER)); // required to use this through a raw pointer or use #[allow(static_mut_refs)] (currently commented out at top of func)
    }
}

// unsafe traits:
unsafe trait Foo {}
unsafe impl Foo for i32 {}



// Example needing to mark Send & Sync for unsafe traits
struct MyBox {
    ptr: *mut i32,
}

impl MyBox {
    fn new(value: i32) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(value)),
        }
    }

    fn get(&self) -> i32 {
        unsafe { *self.ptr }
    }
}

// assert MyBox satisfies Send/Sync
unsafe impl Send for MyBox {} // Send means a value can be moved to another thread
unsafe impl Sync for MyBox {} // Sync means a value can be borrowed (shared by reference) accross threads

impl Drop for MyBox {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.ptr)) }
    }
}

fn unsafe_traits() {
    // traits are unsafe if they have some invariant that the compiler can't confirm
    let box_a = MyBox::new(10);

    // spawn a new thread - std::marker::Send used here
    thread::spawn(move || println!("{}", box_a.get()))
        .join()
        .unwrap();

    let box_b = MyBox::new(20);

    // scope a new thread - std::marker::Sync used here
    // box_b.get borrows
    thread::scope(|s| { // create a temporary scope - allows borrowing box_b and &box_b accross threads
        s.spawn(|| println!("{}", box_b.get()));
        s.spawn(|| println!("{}", box_b.get()));
    });
}
