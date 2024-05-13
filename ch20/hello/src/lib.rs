use std::thread;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Creates a new Worker
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Creates a new Thread Pool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create threads and store them in the vector
            workers.push(Worker::new(id));
        }
        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}
