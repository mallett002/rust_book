use std::ops::Deref;

fn main() {
    following_ref_to_value();
    using_box_like_ref();
    defining_own_smart_pointer();
    deref_coercion();

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

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn deref_coercion() {
    let str_slice = "Rust";
    hello(str_slice);

    // Deref coercion allows us to call hello with ref to MyBox<String>
    // because MyBox implements Deref trait
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // The code we would have to write if MyBox didn't implment Deref
    let myBoxedStr = MyBox::new(String::from("Rust"));
    
    let derefed = &(*myBoxedStr)[..];
    // *myBoxedStr -> turns MyBox<String> into String
    // [..] and & ->  takes string slice (&str) of the whole string 

    hello(derefed);
}
