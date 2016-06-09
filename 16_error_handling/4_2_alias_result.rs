// when we use a specific result many times, we can use an alias
use std::num::ParseIntError;
use std::result;

// define generic alias for a Result with error type ParseIntError
type AliasedResult<T> = result::Result<T, ParseIntError>;

// use the alias above
fn double_number(number_str: &str) -> AliasedResult<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

// here the alias again allows us to save some space
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(double_number("10"));
    print(double_number("t"));
}