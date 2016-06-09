// vars are immutable by default, but can be changed with the mut keyword
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("before mut: {}", mutable_binding);

    // ok
    mutable_binding += 1;

    println!("after mut: {}", mutable_binding);

    // not ok
    // _immutable_binding += 1;
}
