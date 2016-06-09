// derive the fmt::Debug implementation for Structure, which contains a single i32
#[derive(Debug)]
struct Structure(i32);

// put a Structure inside the struct Deep, and make it printable also
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // printing with {:?} is similar to with {}
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // Structure is now pritable
    println!("Now {:?} will print!", Structure(3));

    // problem with derive is that there is no control over how the results look
    println!("Now {:?} will print!", Deep(Structure(7)));

    // solution is to manually implement fmt::Display
}
