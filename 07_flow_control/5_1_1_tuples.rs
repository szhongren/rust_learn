// tuples can be destructured in a match as below
fn main() {
    let pair = (0, -2);
    println!("Tell me about {:?}.", pair);
    // match can be used to destructure a tuple
    match pair {
        (0, y) => println!("First is 0 and y is {:?}", y),
        (x, 0) => println!("x is {:?} and last is 0", x),
        _ => println!("It doesn't matter what they are."),
    }
}
