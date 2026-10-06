use std::{thread, time::Duration};

fn main() {
    // creating_new_thread();
    using_move_closures_with_threads();
}

fn creating_new_thread() {
    // use thread::spawn to create new thread

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hello {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hello {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // cause main thread to block and wait until spawned thread finishes
    handle.join().unwrap();
}

fn using_move_closures_with_threads() {
    let v = vec![1, 2, 3];

    // use the "move" keyword to take ownership of v instead of borrow (ref)
    let handle = thread::spawn(move|| {
        println!("here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
