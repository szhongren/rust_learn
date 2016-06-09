// since closures can be args, functions can also be args. only difference being a function can never capture variables, so closures are more flexible

// a fn that takes a closure as an arg and calls it
// F is of type Fn(), and f is of type F
fn call_function<F: Fn()>(f: F) {
    f()
}

fn print() {
    println!("I'm a function! I can be used like a closure.")
}

fn main() {
    let closure = || println!("I'm a closure!");

    call_function(closure);
    call_function(print);
}
