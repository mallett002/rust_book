use std::pin::{Pin, pin};
use std::task::{Context, Poll};
use std::time::Duration;

fn main() {
    join_many_futures();
    stream_trait();
}

fn join_many_futures() {
    // using trpl::join_all to await many unknown futures

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // Clone tx to have multiple senders
        let tx1 = tx.clone();

        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();

                // Yield control to the async runtime here,
                // allowing other tasks to make progress.
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(val) = rx.recv().await {
                println!("received '{val}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();

                // Yield control to the async runtime here,
                // allowing other tasks to make progress.
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        // Pin the futures so they cannot move in memory if they contain self-references.
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        // The join_all takes ownership of the vector, but moving the
        // pinned references doesn't move the underlying futures.
        // Pin allows us to use futures that may not implement Unpin.
        // Unpin means a value can safely be moved in memory.
        // .await polls a future; it doesn't automatically pin every future.
        // join_all requires each item to implement Future.

        trpl::join_all(futures).await;

        // Unpin: The value can safely move around in memory.
        // Pin: Guarantees a value won't move in memory after being pinned (if it isn't Unpin).
        // Why? Some async futures contain references to their own data. Moving them could break those references.
        // Your code: pin! pins each future so join_all can work with them safely.
        // Remember: Pin is about memory location, not about whether a future is running or paused.
    });
}

// Example Stream trait def similar to one in futures crate:
// Stream trait that combines Iterator and Future
// Iterator that gets the next Item and returns it as an option
// Future that polls for and returns a Poll enum to see if it's ready
trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}

// StreamExt: uses await for the "next" method instead of poll_next (much nicer)
// StreamExt automatically implemented for every trait that implements Stream
trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;

    // other methods...
}

fn stream_trait() {
    // see traits above    

    // Self::Item — the type of each value produced.
    // Option — Some(item) means a value; None means the stream is finished.
    // Poll — Ready(...) means the answer is available now; Pending means not ready yet.
    // Pin<&mut Self> — allows polling without moving a stream that must stay in place.
    // Context — lets the async runtime know how to wake the task when it should try again.

    // Example: A stream of chat messages might produce "Hi", then "Hello", then "How are you?" over time, and finally None.
    // You usually use .next().await through StreamExt instead of calling poll_next yourself.
    // The rust book (https://doc.rust-lang.org/book/ch17-05-traits-for-async.html#the-stream-trait) ..
    // ..defines a simplified Stream trait to explain the concept; the widely used version comes from the futures crate.
}
