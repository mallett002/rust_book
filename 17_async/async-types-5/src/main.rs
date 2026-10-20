use std::time::Duration;

fn main() {
    join_many_futures();
}

fn join_many_futures() {
    // using trpl::join_all to await many unknown futures
    //

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();    

        // clone tx to have multiple senders
        let tx1 = tx.clone();

        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                // hand control to async runtime here:
                // runtime looks for other tasks - rx_fut to process
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
               println!("received '{val}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                // hand control to async runtime here:
                // runtime looks for other tasks - rx_fut to process
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // Box bc Vec needs concrete type, but future may have diff concrete type
        let futures: Vec<Box<dyn Future<Output = ()>>> = 
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        // Put inside Pin type - ensure they don't move in memory
        // Required bc `dyn Future<Output = ()>` needs to implement the Unpin trait
        // Awaiting a future with .await implicitly pins it
        // join_all returns JoinAll - requires all items implement Future trait
        // Box<T> implements future only if T is a future that implements Unpin trait

        // TODO: left off https://doc.rust-lang.org/book/ch17-05-traits-for-async.html
        // See: "The trpl::join_all function returns a struct called JoinAll"

        trpl::join_all(futures).await;
    });
}
