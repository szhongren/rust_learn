// panicking on unwrap gives an unhelpful message, so we need to be more specific about the return type
// Err type in the previous example is ParseIntError
// parse() is implemented with the FromStr trait for i32
// map method is also implemented for Result
use std::num::ParseIntError;

// with return type rewritten, we can use match without unwrap but it is tedious
fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(e) => Err(e),
    }
}

// as with Option, we can use combinators like map()
// this fn is otherwise identical to the above
fn double_number_map(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // this still presents a reasonable answer
    let twenty = double_number("10");
    print(twenty);

    // the following now provides a much more useful error message
    let tt = double_number_map("t");
    print(tt);
}