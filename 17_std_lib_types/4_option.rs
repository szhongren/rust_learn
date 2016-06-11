// sometimes we want to catch the failure somewhere in a program instead of
// using panic! so we can use Option
// 2 variants:
// - None, to indicate failure or lack of value
// - Some(value), a tuple struct that wraps a value with type T
// integer division that doesn't panic!
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // failure represented by None
        None
    } else {
        // result is wrapped in a Some variant
        Some(dividend / divisor)
    }
}

// this fn handles a division that may not succeed
fn try_division(dividend: i32, divisor: i32) {
    // Option vals can be matched, like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // binding None to a var needs to be type annotated
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // unwrapping Some will extract the value in it
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // unwrapping None will panic!
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}