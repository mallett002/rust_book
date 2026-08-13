use std::cmp::PartialOrd;

fn main() {
    // duplicated();
    // extract_out_to_dedupe();
    // duplicated_int_char();
    de_duped_with_generics();
    generics_with_structs();
}

fn duplicated() {
    let largest = vec![34, 50, 25, 100, 65];

    let mut largest_num = &largest[0];

    for num in &largest {
        if num > largest_num {
            largest_num = num;
        }
    }

    println!("largest: {largest_num}");

    // find largest in 2 lists
    let largest = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest_num = &largest[0];

    for num in &largest {
        if num > largest_num {
            largest_num = num;
        }
    }

    println!("largest: {largest_num}");
}

fn extract_out_to_dedupe() {
    // extract finding largest to a function
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("largest: {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("largest: {result}");
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn duplicated_int_char() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}

fn de_duped_with_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_generics(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_generics(&char_list);
    println!("The largest char is {result}");
}

// same logic inside function, but now with generic type definition
// PartialOrd enforces that T can be compared with ">"
fn largest_generics<T: PartialOrd>(list: &[T]) -> &T { 
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn generics_with_structs() {
    let int_point = Point{ x: 5, y: 10 };
    let float_point = Point{ x: 1.0, y: 4.0 };
    let mixed_point = Point{ x: 1, y: 4.0 };
}

// Generics on an enum:
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// TODO: left off https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions

