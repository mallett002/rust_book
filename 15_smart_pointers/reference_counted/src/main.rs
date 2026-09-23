// There are times when you can actaully have multiple owners of a value
// We use Rc<T> to manage that

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>), // Rc so now multiple owners allowed
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    trying_to_use_box_t();
    use_ref_counting_instead();
}

fn trying_to_use_box_t() {
    /*
    *
    enum List {
        Cons(i32, Box<List>), // Rc so now multiple owners allowed
        Nil,
    }

    * B has 3 then A's contents
    * C has 4 then A's contents

    b --> 3
          |
          V
          a --> 5, 10
          ^
          |
    c --> 4
    */

    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a)); // b takes ownership of a here
    // let c = Cons(4, Box::new(a)); // can't use a anymore
}

fn use_ref_counting_instead() {
    /*
    * B has 3 then A's contents
    * C has 4 then A's contents

    b --> 3
          |
          V
          a --> 5, 10
          ^
          |
    c --> 4
    */

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // another reference to a (watching TV)
    let c = Cons(4, Rc::clone(&a)); // another reference to a (watching TV)

    // Rc::clone doesn't take deep copy of data, only increments reference count
    // unlike other "clone" methods, which typically do a deep copy

    // a won't be cleaned up until no more references to it
    // i.e. no one is watching the TV anymore.
}
