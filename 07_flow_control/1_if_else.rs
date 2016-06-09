// if else is c-like, but each condition is followed by a block and all branches are expressions that must return the same type
fn main() {
    let n = 5;

    if n < 0 { // python like
        print!("{} is negative", n); // each branch needs the curly braces though
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number. Increase 10-fold.");
            // this exp returns an i32
            10 * n
        } else {
            println!(", and is a big number. Reduce by 2.");
            n - 2
        }; // end of let binding

        println!("{} -> {}", n, big_n);
}
