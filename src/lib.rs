use std::thread;
use std::{sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


struct Job;


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

        let (sender,receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {workers, sender}
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver
        });

        Worker {id, thread}
    }
}