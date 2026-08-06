use std::fs::File;
use std::io::{Error, Read};

fn main() {
    // panicking();
    recoverable_errors();
}

fn panicking() {
    // one way to panic
    panic!("crash and burn"); 

    // another way to panic
    let v = vec![1, 2, 3];

    v[99];
}

fn recoverable_errors() {
    // try to open a file
    let greeting_file_result = File::open("hello.txt"); // Result<File, Error> (don't inferred)

    // handle file or error
    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    // read the file contents
    let mut buff = String::new();
    let byte_count_result = greeting_file.read_to_string(&mut buff);

    let byte_count = match byte_count_result {
        Ok(b) => b,
        Err(e) => panic!("Problem reading the file: {e:?}"),
    };

    println!("read {byte_count} bytes");
    println!("buff: {buff}");
}
