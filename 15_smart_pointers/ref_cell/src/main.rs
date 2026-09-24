// Interior mutability: mutate data even when there are immutable references it.

/*
*   RefCell<T>:
*       - How we implement the interir mutability pattern
*       - Can only have 1 owner
*       - Checked at runtime, not compile time
*       - Can have mutable and immutable refs
*       - RefCell<T> itself is immutable, but can mutate values inside it
*/
fn main() {
    // If have immutable value, can't borrow it mutably:
    let x = 5;
    // let y = &mut x; // cannot borrow `x` as mutable


}
