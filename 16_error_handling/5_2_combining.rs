// what if multiple Results need to interact together
use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

// setup to make this work
// create 2 files with some info, ignore the return values
fn setup() {
    File::create("a")
        .and_then(|mut file| file.write_all(b"grape"))
        .unwrap();

    File::create("b")
        .and_then(|mut file| file.write_all(b"fruit"))
        .unwrap();
}

// get the data from each file with the data stored in a Result
fn get_data(path: &str) -> Result<String> {
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();

            // read the data into contents
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                // ignore the output read_to_string returns and return contents
                .map(|_| contents)
        })
}

fn concat(filename_a: &str, filename_b: &str) -> Result<String> {
    let (data_a, data_b) = (get_data(filename_a), get_data(filename_b));

    data_a.and_then(|a|
        // return Ok when beth a and b are Ok
        // otherwise return whichever has the first Err
        data_b.and_then(|b| Ok(a + &b))
    )
}

fn main() {
    setup();

    match concat("a", "b") {
        Ok(n) => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}