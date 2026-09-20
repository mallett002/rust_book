use std::ops::Deref;

fn main() {
    following_ref_to_value();
    using_box_like_ref();
    defining_own_smart_pointer();

    // TODO: left off https://doc.rust-lang.org/book/ch15-02-deref.html#using-deref-coercion-in-functions-and-methods
}

fn following_ref_to_value() {
    let x = 5;
    let y = &x; // ref to x

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn using_box_like_ref() {
    let x = 5;
    let y = Box::new(x); // instance of box pointing to copied val of x

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// MyBox implements Deref
impl<T> Deref for MyBox<T> {
    type Target = T; // Define the type for Deref trait to use

    // deref returns ref to val we want to access with "*" pointer
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn defining_own_smart_pointer() {
    // MyBox doesn't store data on heap like actual Box
    
    let x = 5;
    let y = MyBox::new(x); // instance of box pointing to copied val of x

    assert_eq!(5, x);
    assert_eq!(5, *y); // rust runs this code: `*(y.deref())`
}
