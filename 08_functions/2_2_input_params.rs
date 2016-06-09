// rust chooses how to capture vars on the fly without annotation, which is convenient but this cannot be allowed when writing functions as it is ambiguous.
// the type of capture a closure uses is annotated as one of the following traits:
// 1. Fn: takes captures by reference (&T)
// 2. FnMut: takes captures by mutable reference (&mut T)
// 3. FnOnce: takes captures by value (T)
// however, any annotated parameter restricts capture to itself and above, as in the list above. rust will also preferentially capture variables in the least restrictive mannor possible on a variable-by-variable basis

// fn that takes a closure as an arg and calls it, returns nothing or unit value
fn apply<F>(f: F)
// closure takes no input and returns nothing, captures by value (or above)
where F: FnOnce() { // where here just improves readability

    f()
}
// same as apply<F: FnOnce() -> ()>(f: F)

// fn that takes a closure as an arg and returns an i32
fn apply_to_3<F>(f: F) -> i32
// closure takes an i32 and returns an i32, captures by reference
where F : Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // non-copy type
    let mut farewell = "goodbye".to_owned(); // creates own data from borrowed data, usually by cloning

    // capture 2 vars, greeting by reference and farewell by value
    let diary = || {
        // greeting is by reference, requires Fn
        println!("I said {}.", greeting);

        // mutation forces farewell to be captured by mutable ref, now requires FnMut
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // manually calling drop forces farewell to be captured by value. now requireds FnOnce
        mem::drop(farewell);
    };

    // call fn which applies closure
    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
// basically, depending on the closures that might be passed to a function in a program, we have to annotate the closure capture type as one of the above
