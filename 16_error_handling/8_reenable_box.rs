// implementing Display and From allows us to us almostall of the std lib error handling tools, oxcept for the ability to box our data types
// std lib will automatically convert from any type which implements the Error trait into the trait object Box<Error> via From, which allows the following
// fn foo(...) -> Result<T, Box<Error>> { ... }
// a user may use a variety of libs which each have their own error types, so to define a valid Result<T, E> type a user can
// - define a new wrapper error types around the external lib error types
// - convert to String or some other intermediate choice
// - box into Box<Error> with type erasure

// Boxing it is a common choice. The only penalty is that the underlying error type is only known at runtime and not statically determined. All that needs to be done to enable this is implement the Error trait

use std::error;
use std::fmt;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError {
    fn description(&self) -> &str {
        match *self {
            // A very short description of the error. Doesn't need to be the
            // same as `Display`.
            DoubleError::EmptyVec => "empty vectors not allowed",
            // This already impls `Error`, so defer to its own implementation.
            DoubleError::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // No underlying cause so return `None`.
            DoubleError::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
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