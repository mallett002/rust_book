// Override Drop trait implementation
// Can choose what will happen when a trait is about to go out of scope
// Release resources like files and network connections
use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // Rust calls this drop method when value about to get dropped
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with `{}`", self.data);
    }
}

fn main() {
    // See when Rust calls drop method on Drop trait
    let c = CustomSmartPointer {
        data: String::from("stuff"),
    };

    // Rust won't let you call the drop method manually like this
    // c.drop();
    // Prevent double freed error

    // If you'd want to call drop manually, need to use std::mem::drop
    drop(c);

    let d = CustomSmartPointer {
        data: String::from("more stuff"),
    };

    println!("CustomSmartPointer created");

    // TODO: left off https://doc.rust-lang.org/book/ch15-04-rc.html
}
