// function signatures with lifetimes have a few constraints
// any ref has an annotated lifetime
// any returned ref has same lifetime as input or 'static
// returning refs without input is banned if it will result in returning refs to invalid data
// 1 input ref with lifetime 'a
fn print_one<'a>(x: &'a i32) {
    println!("print_one: x is {}", x);
}

// mutable refs are possible with lifetimes as well
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// multiple elements with different lifetimes
// below, both can use 'a but in certain cases we might need different lifetimes
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {}, y is {}", x, y);
}

// returning refs that have been passed in is acceptable
// correct lifetime must be returned
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

// fn invalid_output<'a>() -> &'a i32 { &7 }
// above is invalid because 'a must live longer than the function
// &7 is destroyed when function returns
fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}