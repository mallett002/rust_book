use trait_objects_2::{Draw, Screen};

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // one way to have different types in a vec is to have an enum like SpreadsheetCell
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("{row:?}");

    // Using trait_objects_2 library to create a screen and call run to draw all components
    // Screen's components only care that each component implements the Draw trait (has draw method)
    // Can be a SelectBox or a Button, or any other type that implements the Draw method
    // Trait objects offer this flexibility
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // Box::new(String::from("Won't work - doesn't implement Draw trait")),
        ],
    };

    screen.run();

    // NOTE: trait object flexibilty incurs a performance cost.
    // Uses dynamic dispatch (Rust doesn't know which method to call until at runtime)
}

// Implementing the trait - what a user might do with our library
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

// TODO: https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html
