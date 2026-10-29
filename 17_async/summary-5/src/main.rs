use std::{thread, time::Duration};

fn main() {
    // When to use threads vs async tasks:
    //
    // When work very parallelizable (CPU-bound) such as processing a bunch of data, where each
    // part can be processed separately, use threads
    //
    // When work very concurrent (I/O-bound)
    //
    // Common pattern where we handle cpu-bound intensive tasks with threads
    // and send message on an async queue to be handled by an async task

    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::block_on(async {
        while let Some(msg) = rx.recv().await {
            println!("received {msg}");
        }
    });
}
