// you can break or continue outer loops when in a nested loop, but the loops must be annotated with some 'label, which must be passed to the break or continue
#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop.");

        'inner: loop {
            println!("Entered the inner loop.");

            // this would break only the inner loop
            break 'outer;
        }
        println!("This point will never be reached.");
    }
    println!("Exited the outer loop.");
}
