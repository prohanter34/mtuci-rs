use std::thread;
use std::sync::{mpsc, Arc, Mutex};
type Job = Box<dyn Send + FnOnce() + 'static >;
pub struct ThreadPool {
    threads: Vec<Worker>,
    tx:Option<mpsc::Sender<Job>>
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.tx.take());
        for worker in &mut self.threads {
            // println!("Shutting down worker {}", worker.id);
            let thread = worker.thread.take();
            match thread {
                Some(s) => {
                    println!("Thread num {} shutdown",worker.id);
                    s.join().unwrap()
                },
                None => {println!("Error none thread")},
            };
        }
    }
}
struct Worker {
    thread:Option<thread::JoinHandle<()>>,
    id:usize
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop{
            let job = receiver.lock().unwrap().recv();
            
            match job {
                Ok(job) => {
                    println!("Worker {} take a job", id);
                    job();
                },
                Err(_) => {
                    println!("Thread {} is shutting down", id);
                    break;
                },
            }

            println!("Thread num {id} ended");
        });
        Worker{ thread: Some(thread), id}
    }
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (tx,rx) = mpsc::channel();
        let conect= Arc::new(Mutex::new(rx));
        let mut threads = Vec::with_capacity(size);
        for i in 0..size {
            threads.push(Worker::new(i, Arc::clone(&conect)));
        }
        return ThreadPool{ threads, tx:Some(tx) };
    }
    pub fn execute<T>(&self, t:T) 
        where T: FnOnce() + Send + 'static {
        let job = Box::new(t);
        let _ = &self.tx.as_ref().unwrap().send(job).expect("error sending");
        


    }
}