// from prev example, when we call parse, immediate reaction is to map the error from a library error into our new error type
// simple and common operation so eliding it would be convenient but it dous not
// cannot use and_then but can use try!
// try! actually unwraps or returns Err(From::from(err))
// Since From::from is a conversion utility between different types, this means if you try! something where the error is convertible to the return type, it will convert automatically. This means, if we rewrite this example with try! when From::from is implemented for our error type, the map_err will go away:
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

// impl the conversion from ParseIntError to DoubleError. this will be automatically called by try! if a ParseIntError needs converting into a DoubleError
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

// same structure as before but instead of chaining, we try! to get out the inner value immediately
fn double_first(vec: Vec<&str>) -> Result<i32> {
    // still convert to Result by stating how to convert None
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
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

// This is actually fairly clean now. If you compare it with the original panic, it is very similar to replacing the unwrap calls with try! except that the return types are Result and so they must be destructured at the top level.

// However, do not expect error handling of this sort to replace all usage of unwrap in practice. Error handling of this sort tripled our code line count and cannot really be called simple even if this is heavily biased by the small code size. Indeed, moving a 1000 line library from unwrap to more proper error handling might be feasible in an additional 100 lines of code though the necessary refractoring definitely would not be trivial.

// This is a very reasonable place to be. Many libraries might get away with only implementing Display and then adding From on an as needed basis. A serious library though will have users with certain expections about how it should implement error handling. In those cases, the error handling will need to be taken one step further.