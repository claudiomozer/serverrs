use std::{
    thread::{JoinHandle, self},
    sync::mpsc
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    round: usize,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            drop(worker.sender.take());
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    } 
}

impl ThreadPool {
    /// Cria uma nova ThreadPool
    /// 
    /// size é a quantidade de threads ativas no pool
    /// 
    /// # Panics
    /// 
    /// A função `new` causa um panic se o size for igual a 0 
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);

        let mut workers  = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { 
            workers,
            round: 0,
        }
    }

    
    pub fn execute<F>(&mut self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.workers.get(self.round).unwrap().sender.as_ref().unwrap().send(job).unwrap();
        let mut next_round: usize = self.round + 1;

        if next_round >= self.workers.len() {
            next_round = 0;
        }
        self.round = next_round;
    }

}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
    sender: Option<mpsc::Sender<Job>>
}

impl Worker {
    fn new(id: usize) -> Worker {
        let (sender, receiver) = mpsc::channel::<Job>(); 
        
        let thread = thread::spawn(move || loop {
            let message = receiver.recv();

            match message {
                Ok(job) => {
                    println!("Worker {}, executing the job", id);
                    job();
                },
                Err(_) => {
                    println!("Worker {} shutting down", id);
                    break;
                }
            }
        }); 

        Worker {
            id,
            thread: Some(thread),
            sender: Some(sender)
        }
    }
}

