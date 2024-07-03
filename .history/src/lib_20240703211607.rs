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