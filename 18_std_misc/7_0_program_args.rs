// the cli args can be accessed with std::env::args, which returns an Iterator that yields a String for each arg
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // the first arg is the path that was used to call the program
    println!("My path is {}.", args[0]);

    // the rest of the args are the passed cli args
    println!("I got {:?} args:", args.len() - 1);
    for arg in &args[1..] {
        println!("> {:?}", arg);
    }
}