fn main() {
    // immutability
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

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



    // -- types
    // 4 main ones (integers, floating points, booleans and characters)
    // TODO: Left off 'https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types'
}
