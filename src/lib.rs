
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}



impl ThreadPool {
    ///create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if the size zero

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {workers}
    }


    /// Implementation of the function new == build
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     if size < 0 {
    //         return Err("The number of threads is less than the allowed value. 1 or more is required.")
    //     }
    //
    //     Ok(ThreadPool, PoolCreationError)
    // }


    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {id, thread}
    }
}