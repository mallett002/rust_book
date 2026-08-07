use std::fs::File;
use std::io::{Error, ErrorKind, Read};
// adding just io here for demonstration of read_username_from_file fn.
use std::io;

fn main() {
    // panicking();
    // recoverable_errors();
    // recoverable_errors_without_match();
    // shortcuts_for_panic_on_error();
    propogating_errors();
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

    // Try to read file
    // If error, check error type
    // if error type is not found create it else panic
    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}")
            }
        },
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

fn recoverable_errors_without_match() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn shortcuts_for_panic_on_error() {
    // unwrap returns result of Ok(), or panics for us
    let greeting_file = File::open("hello.txt").unwrap();

    // expect() is similar to unwrap(), but you can chose your error message
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // expect() is more common than unwrap()
}

// Let calling code handle Result<String, io::Error>
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(f) => f,
        Err(e) => return Err(e), // Return Result::Err here early
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
       Ok(_) => Ok(username), // read_to_string returns byte count read, we don't care ab that
       Err(e) => Err(e),
    }

    // TODO: left off https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-6
    // Find: "If this function succeeds without any problems, the code that calls this function"
}

fn propogating_errors() {
    read_username_from_file();
}
