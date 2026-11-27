use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

// TODO: left off https://doc.rust-lang.org/book/ch21-02-multithreaded.html#implementing-the-execute-method

// Job has a closure to run
// Workers pull jobs from threadpool
// Worker gets the closure from the job and passes it as the closure for the spawn

// 1. The ThreadPool creates a channel and holds on to the sender.
// 2. Each Worker holds on to the receiver.
// 3. We’ll create a new Job struct that will hold the closures we want to send down the channel.
// 4. The ThreadPool's execute method sends the job it wants to execute through the sender.
// 5. In its thread, the Worker loops over its receiver and executes the closures of any jobs it receives.

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job {}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// struct PoolCreationError {
//     size: usize,
//     message: String,
// }

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // ThreadPool::build(size).unwrap_or_else(|e| panic!("{}", e.message))

        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // Receiver = worker. Multiple workers pulling from queue. 1 worker at a time can mutate
        // receiver. A message processed only once
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     if size <= 0 {
    //         return Err(PoolCreationError {
    //             size,
    //             message: String::from("Pool size required to be larger than zero"),
    //         });
    //     }
    //
    //     Ok(ThreadPool {})
    // }

    // FnOnce: The closure can be called exactly once.
    // Send: The closure can safely be transferred to another thread.
    // 'static: The closure cannot borrow data that might disappear
    // before the thread finishes.
    //
    // 'static does NOT mean the closure lives for the full program.
    // It means any references it contains must be 'static.
    // In practice, this often means the closure owns its captured data.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
    }
}
