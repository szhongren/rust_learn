// attribute to hide warnings for unused code
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // explicitly use each name so they are available without manual scoping
    use Status::{ Poor, Rich };
    // use each name inside work
    use Work::*;

    // equiv to Status::Poor
    let status = Poor;
    // equiv to Work::Civilian
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
