// Shared state concurrency:
// Multiple threads can access the same location in memory at the same time

use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

fn main() {
    control_access_with_mutex();
    shared_access_to_mutex();
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

fn shared_access_to_mutex() {
    // shared access to the counter (mtx_counter)

    // Wrap mtx_counter in Atomic Rc (Arc)
    // To use it in new thread, needs to take ownership and we need 1..10 threads
    // Can't use Rc here. not multi-thread safe. use Arc instead
    let mtx_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let mtx_counter = Arc::clone(&mtx_counter);

        let handle = thread::spawn(move || {
            let mut num = mtx_counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *mtx_counter.lock().unwrap());
}
