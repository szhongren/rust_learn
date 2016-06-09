// var bindings have a scope and are limited to a block
// bindings can also be shadowed, which can be in the same block or nested one or more levels down
fn main() {
    // this binding lives in the main fn
    let long_lived_binding = 1;

    // this is a block, and has a smaller scope than the main fn
    {
        // this binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // this binding shadows the outer one
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // end of block

    // short_lived_binding not in scope here
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // this binding also shadows the previous binding
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
