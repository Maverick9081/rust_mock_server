use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool{
    workers : Vec<Worker>,
    sender : mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job =Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
   
    ///this is a trial documentation
    /// 
    ///  
    /// #panic 
    /// it panics when the given size is less than zero
    pub fn new(size :usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute <F>(&self,f:F)
        where
            F:FnOnce() + Send +'static,
        {
            let job =Box::new(f);

            self.sender.send(Message::NewJob(job)).unwrap();
        }
}
impl Drop for ThreadPool {
    fn drop(&mut self){
        println!("sending termination messages to all workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("shutting down server");

        for worker in &mut self.workers {
            println!{"shutting down worker NO. {}",worker.id}
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id : usize,
    thread : Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new (id : usize,receiver :Arc<Mutex< mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move|| loop {
            let message =receiver.lock().unwrap().recv().unwrap();
            
            match message{
                Message::NewJob(job) => {
                    println!("worker {} got a job",id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} is being terminated",id);

                    break;
                }
            }
        });

        Worker {id,
                thread : Some(thread),
            }
    }
}