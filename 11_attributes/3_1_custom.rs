// some conditionals are implicitly provided by rustc, but custom conditionals must be passed to rustc with the --cfg flag
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}
