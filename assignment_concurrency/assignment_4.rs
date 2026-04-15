use std::sync::{mpsc, Arc, Mutex};
use std::thread;


use std::time::Duration;

use rand::RngExt;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 =-1;

fn main()
{
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    let (number_sender, number_receiver) = mpsc::channel();
    
    let shared_receiver = Arc::new(Mutex::new(number_receiver));
    
    let mut producer_threads = vec![];
    let mut consumer_threads = vec![];
    
    for producer_id in 0..2 
    {
        let sender_copy = number_sender.clone();
        producer_threads.push(thread::spawn(move || {
            producer(producer_id, sender_copy, ITEM_COUNT / 2);
        }));
    }
    
    for consumer_id in 0..3
    {
        let receiver_copy = Arc::clone(&shared_receiver);
        consumer_threads.push(thread::spawn(move || {
            consumer(consumer_id, receiver_copy);
        }));
    }
    
    for producer_thread in producer_threads 
    {
    
        producer_thread.join().unwrap();
    }
    
    for _ in 0..3 
    {
        number_sender.send(TERMINATION_SIGNAL).unwrap();
    }
    
    for consumer_thread in consumer_threads
    {
        consumer_thread.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize)
{
    let mut random_generator = rand::rng();
    
    for _ in 0..item_count 
    {
        let generated_number = random_generator.random_range(1..100);
        
        
        println!("Producer {} produced {}", id, generated_number);
        tx.send(generated_number).unwrap();
        
        thread::sleep(Duration::from_millis(100));
    }
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>)
{
    loop 
    {
        let received_number = rx.lock().unwrap().recv().unwrap();
        
        if received_number == TERMINATION_SIGNAL 
        {
            break;
        }
        
        println!("Consumer {} processing {}", id, received_number);
        
        thread::sleep(Duration::from_millis(150));
    }
}
