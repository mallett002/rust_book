use std::io;

fn tuple_type() {
    // Groups together number of values with multiple types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // can destructure like htis
    let (x, y, z) = tup;

    println!("The value of y is {y}");




    let my_tup: (i32, f64, u8) = (500, 6.4, 1);

    // can access items in tuple directly with a "."index
    let five_hundred = my_tup.0;
    let six_point_four: f64 = my_tup.1;
    let one: u8 = my_tup.2;

    // unit is a tuple without any values
    let my_unit: () = ();
    println!("The value of five_hundred is {five_hundred}");
    println!("The value of six_point_four is {six_point_four}");
    println!("The value of one is {one}");
    println!("The value of my_unit is {my_unit:?}");
}

fn array_out_of_bounds_program() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // could be out of bounds
    let element = a[index];

    println!("The value at element {index} is: {element}");
}

fn array_types() {
    // arrays have fixed length
    // items in array have same type
    // data allocated on stack, not heap
    // vector type is on heap. more flexible, can grow/shrink
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5]; // define with type
    let c = [3; 5]; // creates [3, 3, 3, 3, 3]

    // access
    let first_a = a[0];
    let first_b = b[0];
    let first_c = c[0];
    println!("first a: {first_a}");
    println!("first b: {first_b}");
    println!("first a: {first_c}");

    array_out_of_bounds_program();
}

fn main() {
    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // immutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing (still immutable)
    let y = 5;
    let y = y + 1;
    // shadowing in scope
    {
        let y = y * 2;
        println!("The value of y in scope: {y}");
    }
    // shadowed out of scope (different)
    println!("The value of y out of scope: {y}");

    // shadowing (changing the type)
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces: {spaces}");



    /* -- types
    4 main ones (integers, floating points, booleans and characters)


    --------------------------------------------------------------
    * INTEGERS
    --------------------------------------------------------------
    https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types
    
        space       signed  unsigned
        8-bit       i8      u8
        16-bit      i16     u16
        32-bit      i32     u32
        64-bit      i64     u64
        128-bit     i12     u128
        arch dep    isize   usize (32 or 64bit machine)

        signed can store  −(2^n − 1) to ( 2^n − 1 ) − 1   (n being the amount of bits)
        - so i8 can store −(2^7) to 2^7 − 1 so −128 to 127

        unsigned can store 0 to ( 2^n ) − 1,
            - so: u8 can store ( 2^8 ) − 1 --> 255
        
        handling overflows with methods:
        - wrapping_* (ex: wrapping_add)
        - Return None value if overflow with the checked_* methods
        - Return the value and a Boolean indicating whether there was overflow with the overflowing_* methods.
        - Saturate at the value’s minimum or maximum values with the saturating_* methods.
        

    --------------------------------------------------------------
    * FLOATING POINT NUMBERS
    --------------------------------------------------------------
        f32 & f64 (default, more precise)
        all floating pt numbers are signed

         let myFloat = 2.0; // f64
         let myFloat2: f32 = 2.0; // f32


    --------------------------------------------------------------
    * BOOLEANS
    --------------------------------------------------------------
        type: bool
        let t = true;
        let f: bool = false; // explicit type def


    --------------------------------------------------------------
    * CHAR
    --------------------------------------------------------------
        type: char is 4 bytes in size
        represents unicode scalar (so much more than just ascii)
        
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    */


    /* --------------------------------------------------------------
    * Compound Types
    --------------------------------------------------------------*/
    tuple_type();
    array_types();
}
