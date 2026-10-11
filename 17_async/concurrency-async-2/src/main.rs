use std::time::Duration;

fn main() {
    // spawn_async_task();
    without_spawn_task();
}

fn spawn_async_task() {
    trpl::block_on(async {
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
