// indirectly accessing a variable makes it impossible to branch and use that variable without re-binding. match provides the @ symbol for binding values to names
// fn age() that returns a u32
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are.");

    match age() {
        0 => println!("I'm not born yet I guess."),
        // binding the matched value to n to use the variable in functions
        n @ 1 ... 12 => println!("I'm a child of age {:?}.", n),
        n @ 12 ... 19 => println!("I'm a teen of age {:?}.", n),
        // nothing bound, return the result
        n => println!("I'm an old person of age {:?},", n),
    }
}
