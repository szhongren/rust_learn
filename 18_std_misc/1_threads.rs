// rust provides a mechanism to spawn native OS threads with the spawn function
// takes a moving closure
use std::thread;

static NTHREADS: i32 = 10;

// this is the main thread
fn main() {
    // make a vec to hold the children which are spawned
    let mut children = vec![];

    for i in 0..NTHREADS {
        // spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }

    for child in children {
        // wait for thread to finish, returns a result
        let _ = child.join();
    }
}