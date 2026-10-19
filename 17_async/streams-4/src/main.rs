// https://doc.rust-lang.org/book/ch17-04-streams.html

// Streams are futures
// Iteratores are synchronous
// Channel receiver is asynchronous
// Stream is async form of iteration - very similar to interators
//
// Can create stream from any iterator

use trpl::StreamExt;

fn main() {
    create_stream_from_iterator();
}

fn create_stream_from_iterator() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);

        // create stream from iterator
        let mut stream = trpl::stream_from_iter(iter);

        // consume the stream awaiting next()
        while let Some(n) = stream.next().await {
            println!("The value is {n}");
        }
    });
}
