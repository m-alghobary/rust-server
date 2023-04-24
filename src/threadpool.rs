use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // ThreadPool with zero worker threads make no sense
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        /*
            Each worker thread need access to the receiving part of the channel,
            to allow multilpe threads to own the receiver we use Arc (atomic reference counter).

            And to control the access to the receiver we use Mutex to ensure only one thread has
            access to the receiver at a time.
        */
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // clone the receiver for each worker
            let receiver = Arc::clone(&receiver);

            workers.push(Worker::new(id, receiver));
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
        /*
           The type F: FnOnce() + Send + 'static, is not an actual type, it's just requirements
           for the type of the f parameter, so we need to wrap it in a Box so we can pass it around.
        */
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // droping the sender notify all receiving threads to shutdown
        drop(self.sender.take());

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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            /*
               first we try to get the mutex lock, then after we get the lock we
               block until we receive a job.

               Mutex::lock() return a MutexGuard but because we are not storing it in a variable,
               the guard will be dropped at the end of this line which will release the mutex lock.
            */
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                // mpsc::Receiver<T>::recv() only returns Err if no sender exists anymore,
                // which means no jobs left to receive and this worker can shut down
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
