// rust has 2 different types of constants which can be declared in any scope including global
// 1. const, unchangeable value
// 2. static, a possibly mutable value with 'static lifetime
// one special case is the string literal, which can be directly assigned to a static variable without modification because its type signature &'static str has the required lifetime of 'static
// all other ref types must be specifically annotated to fulfil the 'static lifetime
// static lifetime lasts for the lifetime of the runnig program, and storeed in the read-only memory of the binary
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // access const in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // access const in main thread
    println!("this is {}", LANGUAGE);
    println!("the threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // cannot modify a const, so following line will not work
    // THRESHOLD = 5;

}
