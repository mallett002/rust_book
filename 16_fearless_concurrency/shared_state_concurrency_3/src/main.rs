// Shared state concurrency:
// Multiple threads can access the same location in memory at the same time

use std::sync::Mutex;

fn main() {
    control_access_with_mutex();
}

fn control_access_with_mutex() {
    // mutex: mutual exclusion
    // only allows 1 thread to access memory at a time
    // rules:
    //  - must attempt to aquire lock before using the data
    //  - when done with data, must unlock

    // example
    let m = Mutex::new(5);

    {
        // aquire lock. blocks until our turn to use data:
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    // when num goes out of scope here, lock is released

    println!("m = {m:?}");
}
