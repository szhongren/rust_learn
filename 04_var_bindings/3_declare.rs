// you should usually declare and init variables in the same line but you don't have to
fn main() {
    let a_binding;

    {
        let x = 2;

        // init the binding
        a_binding = x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // cannot use uninitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
