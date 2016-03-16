// fns are declared with fn keyword, and args and return type are type annotated
// final expression in a fn is used as a return val, or the return keyword can be used to return from an earlier point
// unlike c/c++, there is no restriction on the order of function definitions
fn main() {
    // we can use this fn here, and def it later
    fizzbuzz_to(100);
}

// fn that returns a bool
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // corner case, early return
    if rhs == 0 {
        return false;
    }
    // expression that becomes the return value of the n
    lhs % rhs == 0
}

// functions that don't have a return value actually return the unit type ()
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}


// when a fn returns (), the return type can be ommited form the signature
fn fizzbuzz_to(n: u32) {
    for n in 1 .. n + 1 {
        fizzbuzz(n);
    }
}
