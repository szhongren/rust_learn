// macros are like functions but end with a !, and expand into source code that gets compiled with the rest of the program. much like c #define macros. they allow metaprogramming and are created with the macro_rules! macro

// this is a simple macro named say_hello!
macro_rules! say_hello {
    // () means the macro takes no args
    () => (
        // macro will expand into the contents of the block
        println!("Hello!");
    );
}

fn main() {
    // this call will expand into the above
    say_hello!()
}
