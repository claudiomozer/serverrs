use std::{
    thread::{Thread, JoinHandle, self},
    sync::{mpsc, Mutex, Arc}
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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
        let (sender, receiver) = mpsc::channel();

        let receiver: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }

}

type WorkerReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;
struct Worker {
    id: usize,
    thread: JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: WorkerReceiver) -> Worker {
        let thread = thread::spawn(move || { 
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {}, executing the job", id);
                
                job();
            }
        }); 

        Worker {
            id,
            thread
        }
    }
}

