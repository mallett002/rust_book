use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

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
    // example_cycle();
    onwership_relationships();
}

fn example_cycle() {
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

    println!("b rc count: {:?}", Rc::strong_count(&b)); // count: 2
    println!("a rc count: {:?}", Rc::strong_count(&a)); // count: 2

    // This will overflow the stack
    // println!("a next item: {:?}", a.tail());

    // Issue: at end of main, ref count of "a" and "b" will go to 1 (not 0) bc they were dropped
    // So, memory for "a" and "b" won't be dropped (dangling memory leak that won't be cleand up)
    // Can potentially use ownership relatioships to fix this issue
}

// A node owns its children
// Want to share ownership with variables so we can access each node in tree
// So, use Vec<Rc<Node>> as children
// Also, want to modify which nodes are children of other nodes (RefCell)
// children use Rc. A node owns its children
// parent uses Weak, so kids don't keep parents alive
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn onwership_relationships() {
    // Weak count doesn't need to be 0 for Rc<T> to be cleaned up
    // downgrade() creates a weak ref - increases weak_count

    // Parent should own its children (if parent dropped, child dropped)
    // Child shouldn't own parent (if child dropped, parent not)
    // If parent type was Rc<T>, would cause ref cycle. incoming Weak<T>

    // create a leaf node
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()), // leaf doesn't own parent
    });

    // See of leaf has a parent (upgrade called to check if ref exists)
    // Need to call upgrade since a weak ref might not be pointing to anything anymore
    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());

    // create a branch that holds leaf. leaf now has 2 owners
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    // set branch to be owner of leaf
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // get interior mut borrow of parent RefCell<T>
    // set it to a Weak ref to branch

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());

    // TODO: left off https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#visualizing-changes-to-strong_count-and-weak_count
}
