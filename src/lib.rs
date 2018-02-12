//! # Webserver
//!
//! This webserver leverages a thread pool to serve simultaneous connections.
//! It is based on chapter 20 of the
//! [Rust book (second edition)](https://doc.rust-lang.org/book/second-edition/ch20-00-final-project-a-web-server.html).

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Working around FnOnce limitations.  See below link for details.
// https://doc.rust-lang.org/book/second-edition/ch20-05-sending-requests-via-channels.html#sending-requests-to-threads-via-channels
trait FnBox {
    fn call_box(self: Box<Self>);
}

// Working around FnOnce limitations.  See below link for details.
// https://doc.rust-lang.org/book/second-edition/ch20-05-sending-requests-via-channels.html#sending-requests-to-threads-via-channels
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// Working around FnOnce limitations.  See below link for details.
// https://doc.rust-lang.org/book/second-edition/ch20-05-sending-requests-via-channels.html#sending-requests-to-threads-via-channels
type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

/// Pool of thread workers for processing requests.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new [`ThreadPool`](struct.ThreadPool.html).
    ///
    /// # Arguments
    ///
    /// `size` - Number of threads in the pool.
    ///
    /// # Panics
    ///
    /// - If [`size`](struct.ThreadPool.html#argumentsfield.size) is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Sends an incoming request to the pool of workers for processing.
    ///
    /// # Arguments
    ///
    /// # Panics
    ///
    /// # Examples
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Joins all worker threads in [`ThreadPool`](struct.ThreadPool.html) before exiting.
    ///
    /// # Panics
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
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
