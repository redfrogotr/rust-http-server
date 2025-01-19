use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

use super::Job;

pub struct Worker {
  id: String,
  thread: JoinHandle<()>,
}

impl Worker {
  pub fn new(id: String, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let bid = id.clone();
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();
      println!("{bid} process request!");
      job();
    });

    Worker {
      id,
      thread,
    }
  }
}