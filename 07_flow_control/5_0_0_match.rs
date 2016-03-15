// rust match is pattern matching, which can be used like switch but better
fn main() {
    let number = 13;

    println!("Tell me about {}.", number);
    match number {
        // match a single value
        1 => println!("One!"),
        // match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime."),
        // match an inclusive range
        13...19 => println!("A teen."),
        // handle other cases
        _ => println!("Ain't special."),
    }

    let boolean = true;
    // match is an expression too
    let binary = match boolean {
        // the arms of a match must cover all possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
