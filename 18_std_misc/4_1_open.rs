// File struct represents a file that has been opened(wraps a file descriptor)
// and gives read/write access to the underlying file
// since many things can go wrong when doing file I/o, all the file methods return the io::Result<T> type
// which is an alias for Result<T, io::Error>
// thus, all I/O failures are explicit, and the programmer can see all failure paths
// open static method can be used to open a file in read-only mode
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // open file read-only, returns io::Result<File>
    let mut file = match File::open(&path) {
        // the description method of io::Error returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // read file into a string, returns io::Result<usize>
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }


}