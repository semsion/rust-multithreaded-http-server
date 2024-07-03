use std::thread;

pub struct ThreadPool {
  workers: Vec<Worker>,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
        workers.push(Worker::new(id));
    }

    ThreadPool { workers }
}
  
  pub fn execute<F>(&self, _f: F)
  where
      F: FnOnce() + Send + 'static,
  {
  }
}