// Rust provides Higher Order Functions (HOF). These are functions that take one or more functions and/or produce a more useful function. HOFs and lazy iterators give Rust its functional flavor.
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000.");
    let upper = 1000;

    // imperative approach
    // declare accumulator
    let mut acc = 0;
    // iter 0 .. inf
    for n in 0.. {
        // sq the number
        let n_squared = n * n;

        if n_squared >= upper {
            // break if exceeding the limit
            break;
        } else if is_odd(n_squared) {
            // acc val if odd
            acc += n_squared;
        }
    }
    println!("Imperative style: {}", acc);

    // functional approach
    let sum_of_sq_odd_numbers: u32 =
        (0..).map(|n| n * n)                // sq all nums
             .take_while(|&n| n < upper)    // below upper limet
             .filter(|n| is_odd(*n))        // if odd
             .fold(0, |sum, i| sum + i);    // sum all
    println!("functional style: {}", sum_of_sq_odd_numbers);
}
