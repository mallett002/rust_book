use std::time::Duration;

fn main() {
    // spawn_async_task();
    // without_spawn_task();
    // sending_data_btw_tasks_message();
    // msg_received_immediately();
    // fix_msg_received_immediately();
    join_multiple_futures();
    // TODO: left off https://doc.rust-lang.org/book/ch17-03-more-futures.html
}

fn spawn_async_task() {
    trpl::block_on(async {
        // spawn an async task
        // sort of like a light weight thread
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // wait for async spawned task to finish
        handle.await.unwrap();
    });
}

fn without_spawn_task() {
    // async blocks compile to anonymous futures
    // can have the async runtime run them to completion with "join"

    // results below show exact same order every time
    // async runtime is fair (checks each future equally, doesn't let one race ahead)
    // OS's aren't fair.
    // async runtimes often have option to be fair

    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}

fn sending_data_btw_tasks_message() {
    // sending data btw 2 tasks with message passing
    // no need to spawn a separate thread/task
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let val = String::from("hi");

        tx.send(val).unwrap();

        // waits until msg received or send side closes
        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    });
}

fn msg_received_immediately() {
    // with this, all messages come immediatelly
    // we want them to come when they're ready, every .5 sec
    // does everything in this async block sequentially
    // does all the sends, then once finished, does all received
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}

fn fix_msg_received_immediately() {
    // msg_received_immediately - all messages come at same time
    // Fix: put tx & rx in own async block:
    // But, this program still never exits!

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // move here - so tx gets owned by async block
        // then, once dropped, received side will finally be None
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                // hand control to async runtime here:
                // runtime looks for other tasks - rx_fut to process
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // fix: send/recv ran concurrently
        trpl::join(tx_fut, rx_fut).await;
    });
}

fn join_multiple_futures() {
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

        // join macro instead here
        // order futures are awaited, not created that matters:
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}
