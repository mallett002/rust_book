pub trait Draw {
    fn draw(&self);
}

// Components is a vector that holds any type that's
// in a Box that implements the Draw trait.
// More flexible than a trait bound with a "where" T: Draw
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// ex with trait bound
// Restricts us to a Screen all of same type, ex Button or TextField
/*
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw
{
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
*/

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// TODO: left off https://doc.rust-lang.org/book/ch18-02-trait-objects.html#implementing-the-trait
