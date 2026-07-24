fn main() {
    // Vectors can only store values of the same type
    // Puts values next to each other in memory (in heap)

    // Create new empty vector
    let my_vec: Vec<i32> = Vec::new();

    // Create new vector with values
    let mut my_vec = vec![1, 2, 3];

    // Adding items to the vector -------------------------------------------------------------
    my_vec.push(4);
    my_vec.push(5);
    my_vec.push(6);

    // Reading values -------------------------------------------------------------------------
    // via index
    let third: &i32 = &my_vec[2];

    // reading via "get"
    let third: Option<&i32> = my_vec.get(2); // pass index as well

    match third {
        Some(v) => println!("The third element is {v}"),
        None => println!("There is no third element"),
    }

    // out of range ---------------------------------------------------------------------------
    // let does_not_exist = &my_vec[100]; // panics
    let does_not_exist = my_vec.get(100); // doesn't panic (Option::None)

    // ownership -----------------------------------------------------------------------------
    // adding new element to vector might require more space in memory
    // vectors put their items next to each other in memory
    // a whole new space in memory might be needed (like a new table in a restaurant)
    // so rust prevents you from holding both a mutable and immutable borrow
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // hold reference to it (immutable borrow)

    // v.push(6); // mutable borrow occurs here (might have reallocated all the vecs memory and first
    // might be pointed at a new location

    println!("The first element is {first}"); // immutable borrow again used here. no-op after
    // pushing. might not be at the same location

    // TODO: left off: https://doc.rust-lang.org/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector
}
