// we usually use Option type to annotate that absence is a possibility
// when multiple failure points exist, we can use the more general Result type
// Result<T, E> has these variants
// Ok<T> element T was found
// Err<E> error was found with element E
// Result also contains unwrap() method which yields element T or calls panic!
fn double_number(number_str: &str) -> i32 {
    // might not always be possible to parse a string into the other type, so parse() returns a Result which indicates possible failure
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let twenty = double_number("10");
    println!("double is {}", twenty);

    let tt = double_number("t");
    println!("double is {}", tt);
    // panicking here, on an Err leaves an unhelpful error message
}
