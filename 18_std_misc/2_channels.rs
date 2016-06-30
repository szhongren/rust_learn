// rust provides async channels for comms between threads
// channels allow a unidirectional flow of information from the Sender to the Receiver
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc; 
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // channels have 2 endpoints: the Sender<T> and the Receiver<T>
    // T is the type of message to be transferred
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    
    for id in 0..NTHREADS {
        // the sender endpoint can be copied
        let thread_tx = tx.clone();    

        // each thread will send its id via the channel
        thread::spawn(move || {
            // thread takes ownership over thread_tx
            // each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // sending is a non-blocking operation, the thread will continue
            // immediately after sending the message
            println!("thread {} finished", id);
        });
    }

    // here all messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // the recv method picks a message from the channel
        // recv will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    // show the order in which the messages were sent
    println!("{:?}", ids);
}
