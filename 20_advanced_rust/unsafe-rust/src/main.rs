fn main() {
    println!("Hello, world!");
    // Unsafe rust:
    // 1. Dereference a raw pointer.
    // 2. Call an unsafe function or method.
    // 3. Access or modify a mutable static variable.
    // 4. Implement an unsafe trait.
    // 5. Access fields of unions.
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
