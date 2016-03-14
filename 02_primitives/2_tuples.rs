use std::fmt::{Result, Formatter, Display};
// a tuple is a collection of values of diff types, constructed using parens. can be used to return multiple values as they can hold any number of values
// tuples declared with ()
// blocks with {}, which are also for structs and enums
// arrays with []
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let can bind the members of a tuple to vars
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // a long tuple
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // values can be extracted from tuples with tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // tuples can be members of tuples
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // to create one element tuples the comma is required to tell them apart from a literal in parens
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("debug: {:?}", matrix);
    println!("display:\n{}", matrix);
}
