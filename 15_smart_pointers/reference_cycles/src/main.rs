use std::cell::RefCell;
use std::rc::Rc;

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // Example Cycle:

    // Create "a" cons list: Cons(5, Nil)
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial count: {}", Rc::strong_count(&a));
    println!("a next item: {:?}", a.tail());

    // Create "b" cons list: Cons(10, a)
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a count after 'b' creation: {}", Rc::strong_count(&a));
    println!("b initial count: {}", Rc::strong_count(&b));
    println!("b next item: {:?}", b.tail());

    // point "a"'s tail back to "b"
    // now we have:
    //  a: Cons(5, b)
    //  b: Cons(10, a)
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("pointed 'a's tail back to b");

    println!("b rc count: {:?}", Rc::strong_count(&b));
    println!("a rc count: {:?}", Rc::strong_count(&a));

    // This will overflow the stack
    // println!("a next item: {:?}", a.tail());
}
