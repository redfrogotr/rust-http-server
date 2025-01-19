use std::sync::{mpsc, Arc, Mutex};

use super::{worker::Worker, Job};

pub struct ThreadPool {
    // You know `()` is very important, we can't think out it easily.
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let arc_receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(format!("Worker-{i}"), Arc::clone(&arc_receiver)));
        }

        ThreadPool {
            threads,
            sender
        }
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}