// channels
// send data between threads
// transmitter end and receiver end
// channel closed if transmitter or recreiver end are dropped

// mpsc (multiple producer single consumer)
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Howdy");
        tx.send(val).unwrap();
    });

    // recv here blocks until receives val
    // use try_recv if you don't want it to block
    let val = rx.recv().unwrap();

    println!("Received value: \"{val}\"");
}
