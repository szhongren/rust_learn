// attribute to hide warnings for unused code
#![allow(dead_code)]

// create an enum to classify someone
// both names and struct types together spec the variant
// skinny != fat and height(i32) != weight(i32)
enum Person {
    // an enum can either be unit like
    Skinny,
    Fat,
    // like tuple structs
    Height(i32),
    Weight(i32),
    // or like structures
    Info { name: String, height: i32 }
}

// a fn which takes a person enum as an arg and returns nothing
fn inspect(p: Person) {
    // usage of an enum must cover all cases (irrefutable) so a match is used to branch over it
    // basically a better switch
    match p {
        Person::Skinny => println!("Is skinny!"),
        Person::Fat => println!("Is fat!"),
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // destructure info into name and height
        // basically let Person::Info{ name, height } = p
        Person::Info{ name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person = Person::Height(18);
    let danny = Person::Weight(10);
    // to_owned() creates an owned string from a string slice
    let dave = Person::Info{ name: "Dave".to_owned(), height: 72 };
    let john = Person::Fat;
    let larry = Person::Skinny;

    inspect(person);
    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);
}
