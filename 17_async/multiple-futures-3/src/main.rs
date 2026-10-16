use std::{thread, time::Duration};

fn main() {
    // Rust pauses tasks and switches to another if awaited future hasn't finished
    // Rust only pauses async blocks and hands control back to runtime at await points
    // Everything btw await points is synchronous
    // If do stuff inside asynch block without an await point, that future will block other..
    // ..futures from making progress (starving other futures)

    // starving_futures_demo();
    fix_starving_futures();
}

// mimics a long-running & blocking operation
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}ms");
}

fn starving_futures_demo() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");

            // some synchronous work
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);

            // hands control back to runtime
            trpl::sleep(Duration::from_millis(50)).await; 

            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");

            // some synchronous work
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);

            // hands control back to runtime
            trpl::sleep(Duration::from_millis(50)).await;

            println!("'b' finished.");
        };

        trpl::join(a, b).await;
    });
}


fn fix_starving_futures() {
    trpl::block_on(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");

            // yield_now - yield execution back to async runtime
            slow("a", 30);
            trpl::yield_now().await; 
            slow("a", 10);
            trpl::yield_now().await; 
            slow("a", 20);
            trpl::yield_now().await; 

            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");

            // add some await points for the runtime to search for other tasks
            slow("b", 75);
            trpl::yield_now().await; 
            slow("b", 10);
            trpl::yield_now().await; 
            slow("b", 15);
            trpl::yield_now().await; 
            slow("b", 350);
            trpl::yield_now().await; 

            println!("'b' finished.");
        };

        trpl::join(a, b).await;
    });
}
