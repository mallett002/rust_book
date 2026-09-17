// Box<T> is a pointer stored on stack that points to data on the heap.
// It's used when you want to allocate data on the heap rather than the stack.
// https://doc.rust-lang.org/book/ch15-01-box.html#using-boxt-to-point-to-data-on-the-heap

// use a Box<T> here so the next value is actaully just a fixed size pointer on stack
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Box it so it's now just a pointer
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Allocate i32 on the heap:
    let b = Box::new(5);
    println!("{b}");

    // Recursive data
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("quack: {list:?}");

    // TODO: left off https://doc.rust-lang.org/book/ch15-02-deref.html
}
