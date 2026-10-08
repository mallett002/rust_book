// channels
// send data between threads
// transmitter end and receiver end
// channel closed if transmitter or recreiver end are dropped

// mpsc (multiple producer single consumer)
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // intro_channels();
    // transfer_ownership_thru_channels();
    // _sending_mult_values();
    creating_mult_producers();
}

fn _intro_channels() {
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

fn _transfer_ownership_thru_channels() {
    // same contents as intro_channels except print after send
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Howdy");
        tx.send(val).unwrap();

        // can't use here after sent (moved)
        // receiver might drop/modify, so rust prevents it
        // println!("val is {val}");
    });

    let val = rx.recv().unwrap();

    println!("Received value: \"{val}\"");
}

fn _sending_mult_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // can loop over receiver
    for received in rx {
        println!("Got: {received}");
    }
}

fn creating_mult_producers() {
    let (tx, rx) = mpsc::channel();

    // clone tx to make another producer
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // all producers send to this same rx
    for received in rx {
        println!("Got: {received}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        _sending_mult_values();
    }
}
