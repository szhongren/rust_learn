// a Path represents file paths in the underlying file system: posix::Path or windows::Path
// Path can be created from any type that implements BytesContainer trait
// Path in not internally represented as a UTF-8 string, but rather a vector of bytes
use std::path::Path;

fn main() {
    // create a path from an &'static str
    let path = Path::new(".");

    // display method returns a Showable structure
    let display = path.display();

    // join merges a path with a byte container using the OS specific separator, and returns new path
    let new_path = path.join("a").join("b");

    // convert path into string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}