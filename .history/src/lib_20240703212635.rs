use std::{
  sync::{mpsc, Arc, Mutex},
  thread,
};

/// A thread pool that manages a collection of worker threads.
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;


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

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers,
      sender: Some(sender),
    }
  }
  
  /// Executes a job in the thread pool.
  ///
  /// Takes a closure with `FnOnce + Send + 'static` traits, sends it to a worker for execution.
  ///
  /// # Arguments
  ///
  /// * `f` - The closure to execute, no arguments, returns nothing.
  ///
  /// # Panics
  ///
  /// Panics if sending the job to a worker fails.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::ThreadPool;
  ///
  /// let pool = ThreadPool::new(4);
  /// pool.execute(|| println!("Job executed by worker."));
  /// ```
  pub fn execute<F>(&self, f: F)
  where
      F: FnOnce() + Send + 'static,
  {
      let job = Box::new(f);

      self.sender.as_ref().unwrap().send(job).unwrap();
  }
}

/// Implements the `Drop` trait for the `ThreadPool` struct.
///
/// When an instance of `ThreadPool` goes out of scope, this `Drop` implementation is called.
/// It shuts down all the worker threads in the thread pool by joining them.
/// It also drops the sender channel, preventing any further tasks from being submitted to the thread pool.
impl Drop for ThreadPool {
  fn drop(&mut self) {
    drop(self.sender.take());

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

/// Represents a worker that executes jobs received through a channel.
/// Each worker runs in its own thread.
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  /// Creates a new `Worker` instance.
  ///
  /// # Arguments
  ///
  /// * `id` - The unique identifier for the worker.
  /// * `receiver` - The shared receiver for receiving jobs through a channel.
  ///
  /// # Returns
  ///
  /// A new `Worker` instance.
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv();

      match message {
        Ok(job) => {
          println!("Worker {id} got a job; executing.");

          job();
        }
        Err(_) => {
          println!("Worker {id} disconnected; shutting down.");
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

#[cfg(test)]
/// Tests for the ThreadPool implementation.
///
/// These tests ensure that the ThreadPool behaves 
mod tests {
    use super::*;

    /// Tests that a ThreadPool with a non-zero number of threads can be created successfully.
    ///
    /// This test verifies that the `new` function of the ThreadPool struct correctly initializes
    /// the pool with the specified number of workers. It checks that the length of the `workers`
    /// vector matches the number provided to `new`.
    #[test]
    fn thread_pool_creation_non_zero() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    /// Tests that creating a ThreadPool with zero threads causes a panic.
    ///
    /// This test ensures that the ThreadPool cannot be created with a size of zero, as it would
    /// not make sense to have a thread pool with no threads. It expects a panic with a specific
    /// message indicating an assertion failure.    
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn thread_pool_creation_zero() {
        ThreadPool::new(0);
    }

    /// Tests that jobs can be executed by the ThreadPool.
    ///
    /// This test checks the functionality of the `execute` method. It creates a ThreadPool and
    /// uses it to execute two jobs that send messages through a channel. The test verifies that
    /// both messages are received, indicating that both jobs were executed by the pool.    
    #[test]
    fn thread_pool_execute_job() {
        use std::sync::mpsc;
        use std::time::Duration;

        let pool = ThreadPool::new(2);
        let (tx, rx) = mpsc::channel();

        let tx_clone = tx.clone();
        pool.execute(move || {
            tx_clone.send(1).unwrap();
        });

        pool.execute(move || {
            tx.send(2).unwrap();
        });

        // Allow some time for the jobs to be executed
        thread::sleep(Duration::from_secs(1));

        let mut results = vec![];
        for _ in 0..2 {
            results.push(rx.try_recv().unwrap());
        }

        assert!(results.contains(&1));
        assert!(results.contains(&2));
    }
}