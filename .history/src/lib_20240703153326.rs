pub struct ThreadPool;

impl ThreadPool {
  pub fn new(_size: usize) -> ThreadPool {
      ThreadPool
  }
  pub fn execute<F>(&self, _f: F)
  where
      F: FnOnce() + Send + 'static,
  {
  }
}