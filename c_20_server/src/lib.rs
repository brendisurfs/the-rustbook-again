pub struct ThreadPool;
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
            Ok(Self)
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
