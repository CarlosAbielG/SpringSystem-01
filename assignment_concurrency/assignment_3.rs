use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message
{
    IncomingTask(Job),
    Stop,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool
{
    workers: Vec<Worker>,
    message_sender: mpsc::Sender<Message>,
}

impl ThreadPool
{
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool
    {
        assert!(size > 0);
        
        let (message_sender, message_receiver) = mpsc::channel();
        
        let shared_receiver = Arc::new(Mutex::new(message_receiver));
        
        let mut worker_group = Vec::with_capacity(size);
        
        for worker_id in 0..size
        {
            worker_group.push(Worker::new(worker_id, Arc::clone(&shared_receiver)));
        }
        
        ThreadPool
        {
            workers: worker_group,
            message_sender,
        }
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let boxed_task = Box::new(task);
        self.message_sender.send(Message::IncomingTask(boxed_task)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        for _ in &self.workers
        {
            self.message_sender.send(Message::Stop).unwrap();
        }
        
        for worker in &mut self.workers
        {
            if let Some(worker_thread) = worker.thread.take()
            {
                worker_thread.join().unwrap();
            }
        }
    }
}

// Worker struct represents a thread that can process jobs
struct Worker
{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker
{
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker
    {
        let worker_thread = thread::spawn(move ||
        {
            loop
            {
                let received_message = receiver.lock().unwrap().recv().unwrap();
                
                match received_message
                {
                    Message::IncomingTask(current_task) =>
                    {
                        println!("Worker {} processing job", id);
                        current_task();
                    }
                    Message::Stop =>
                    {
                        break;
                    }
                }
            }
        });
        
        Worker
        {
            id,thread: Some(worker_thread),
        }
    }
}

fn main()
{
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for task_number in 1..=10
    {
        pool.execute(move ||
        {
            println!("Processing task {}", task_number);

            thread::sleep(std::time::Duration::from_millis(500));


            println!("Completed task {}", task_number);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}