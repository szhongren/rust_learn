// prev problem was awkward because avoiding unwrap forces deeper nesting
// when Err is found, we can panic! or return
// try! is like unwrap, but it returns instead of panics when Err found
use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

// create 2 files with some info
fn setup() -> std::io::Result<()> {
    let mut a = try!(File::create("c"));
    try!(a.write_all(b"lock"));

    let mut b = try!(File::create("d"));
    b.write_all(b"step")
}

// get data from files into a result
fn get_data(path: &str) -> Result<String> {
    // try unwraps the value or returns the error
    let mut file = try!(File::open(path)
        // errors must still be converted to strings
        .map_err(|err| err.to_string())
    );
    let mut contents = String::new();

    // read data into contents
    try!(file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())
    );

    Ok(contents)
}

// concat the contents of the 2 files together into a new Result
fn concat(a: &str, b: &str) -> Result<String> {
    let (data_a, data_b) = (try!(get_data(a)), try!(get_data(b)));

    Ok(data_a + &data_b)
}

fn main() {
    setup().unwrap();

    match concat("c", "d") {
        Ok(n) => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}