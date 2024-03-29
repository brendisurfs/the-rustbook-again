use std::thread;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
};
impl Worker {
    pub fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});
        Self {id, thread}
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
}
pub enum PoolCreationError {
    SizeTooSmall,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        if size > 0 {

            let workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id));
            }

            Ok(Self {
                workers,
            })
        } else {
            Err(PoolCreationError::SizeTooSmall)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
