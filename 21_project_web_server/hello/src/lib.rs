use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
        // TODO: left off https://doc.rust-lang.org/book/ch21-02-multithreaded.html#sending-code-from-the-threadpool-to-a-thread
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
