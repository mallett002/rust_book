use std::{thread, time::Duration};

fn main() {
    creating_new_thread();
    // TODO: left off https://doc.rust-lang.org/book/ch16-01-threads.html#using-move-closures-with-threads
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
