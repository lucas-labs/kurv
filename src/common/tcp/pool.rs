use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// The ThreadPool struct holds a vector of workers and a
/// channel to send them jobs.
pub struct ThreadPool {
    workers: Vec<PoolWorker>,
    sender: Option<mpsc::Sender<Job>>,
}

/// A Job is a boxed closure that holds a function
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(PoolWorker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct PoolWorker {
    thread: Option<thread::JoinHandle<()>>,
}

impl PoolWorker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> PoolWorker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => job(),
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        PoolWorker {
            thread: Some(thread),
        }
    }
}
