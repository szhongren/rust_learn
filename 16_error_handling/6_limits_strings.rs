// using Strings as errors is fine, but does not fulfil all the criteria for a good error type
// - represents different errors with same type
// - presents nice error message to user
// - easily type comparable
// - can hold information about the error
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
// define our error types, which can be customized however we like
enum DoubleError {
    // we don't need extra info to detail this error
    EmptyVec,
    // defer to parse error implementation for their error
    Parse(ParseIntError),
}

// How the type is displayed is completely separate from where the errors are generated.
// We do not need to be concerned that the display style will clutter the complex logic
// our utility requires. They are separate matters which are handled separately.
//
// We don't store extra info about the errors. If we had desired, for example, to state
// which string failed to parse then we can't without modifying our types to carry that
// information accordingly.
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
            // this is a wrapper to defer to underlying types' implementations of fmt
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
    // change error to our new type
       .ok_or(DoubleError::EmptyVec)
       .and_then(|s| s.parse::<i32>()
            .map_err(DoubleError::Parse)
            .map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}