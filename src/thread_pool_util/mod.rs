pub mod thread_pool;

pub mod worker;
type Job = Box<dyn FnOnce() + Send + 'static>;