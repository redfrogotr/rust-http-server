use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

use super::Job;

pub struct Worker {
    pub id: String,
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: String, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let bid = id.clone();
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {bid} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {bid} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
