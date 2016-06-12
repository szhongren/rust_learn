// The panic! macro can be used to generate a panic and start unwinding its stack.
// While unwinding, the runtime will take care of freeing all the resources owned by the thread by calling the destructor of all its objects.

// Since we are dealing with programs with only one thread, panic! will cause the program to report the panic message and exit.
// reimplement integer division
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

fn main() {
    let _x = Box::new(0i32);

    // this op will trigger a task failure
    division(3, 0);

    println!("This point won't be reached");
}