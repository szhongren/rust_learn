// use loop keyword to indicate an infinite loop
// break exits entire loop anytime
// continue skips rest of iteration and starts the loop anew
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // skip the rest of the iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough");

            // exit loop
            break;
        }
    }
}
